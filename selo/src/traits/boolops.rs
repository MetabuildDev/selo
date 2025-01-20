use std::iter::once;

use bevy_math::{DVec2, Vec2};
use i_overlay::{
    core::{fill_rule::FillRule, overlay_rule::OverlayRule},
    float::{overlay::FloatOverlay, source::resource::OverlayResource},
    i_float::float::{compatible::FloatPointCompatible, number::FloatNumber},
    i_shape::base::data::Contour,
};
use sealed_helper_traits::IntoOverlayResource;

use crate::{MultiPolygon, MultiRing, Point2, Polygon, Ring, Triangle};

const FILL_RULE: FillRule = FillRule::EvenOdd;

/// Boolean Operations trait for geometries. These are basic logical operations but for geometry.
/// If a geometry is defined by `{ x | x in geometry }`, then these operations allow to combine two
/// such sets to get a compound set.
///
/// ### Example
///
/// Let
///
/// - `A = { x | x in unit square }`
/// - `B = { x | x in unit circle }`
///
/// then
///
/// - `A \ B = { x | x in unit square but not in unit circle }`
///
/// The trait currently supports
///
/// - `a AND b` = `intersection` = points included in both sets
/// - `a OR b` = `union` = points included in either set
/// - `a AND (NOT b)` = `difference` = points included in first set but not the second set
pub trait BoolOps<Rhs>
where
    Self: IntoOverlayResource + Sized,
    Rhs: IntoOverlayResource<P = Self::P>,
{
    /// Union boolean operation. This creates a combined [`MultiPolygon`] out of the two input
    /// geometries.
    ///
    /// ```
    /// # use selo::prelude::*;
    /// let ring_points = [Vec2::ZERO, Vec2::X * 0.5, Vec2::X * 0.5 + Vec2::Y, Vec2::Y];
    /// let ring1 = Ring::new(ring_points);
    /// let ring2 = Ring::new(ring_points.map(|pos2| pos2 + Vec2::X * 0.5));
    ///
    /// let union = ring1
    ///     .to_polygon()
    ///     .to_multi()
    ///     .union(&ring2.to_polygon().to_multi());
    ///
    /// assert_eq!(union.len(), 1);
    /// assert_eq!(union.area(), 1.0);
    /// ```
    fn union(&self, rhs: &Rhs) -> MultiPolygon<Self::P> {
        boolops(self, rhs, OverlayRule::Union)
    }

    /// Intersection boolean operation. This creates the overlap [`MultiPolygon`] out of the two input
    /// geometries.
    ///
    /// ```
    /// # use selo::prelude::*;
    /// let ring_points = [Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y];
    /// let ring1 = Ring::new(ring_points);
    /// let ring2 = Ring::new(ring_points.map(|pos2| pos2 + Vec2::X * 0.5));
    ///
    /// let intersection = ring1
    ///     .to_polygon()
    ///     .to_multi()
    ///     .intersection(&ring2.to_polygon().to_multi());
    ///
    /// assert_eq!(intersection.len(), 1);
    /// assert_eq!(intersection.area(), 0.5);
    /// ```
    fn intersection(&self, rhs: &Rhs) -> MultiPolygon<Self::P> {
        boolops(self, rhs, OverlayRule::Intersect)
    }

    /// Difference boolean operation. This creates the [`MultiPolygon`] that results from
    /// subtracting the overlap of the two input geometries from the first input geometry.
    ///
    /// ```
    /// # use selo::prelude::*;
    /// let ring_points = [Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y];
    /// let ring1 = Ring::new(ring_points.map(|pos2| pos2 * 2.0));
    /// let ring2 = Ring::new(ring_points);
    ///
    /// let difference = ring1
    ///     .to_polygon()
    ///     .to_multi()
    ///     .difference(&ring2.to_polygon().to_multi());
    ///
    /// assert_eq!(difference.len(), 1);
    /// assert_eq!(difference.area(), 3.0);
    /// ```
    fn difference(&self, rhs: &Rhs) -> MultiPolygon<Self::P> {
        boolops(self, rhs, OverlayRule::Difference)
    }
}

fn boolops<Lhs: IntoOverlayResource, Rhs: IntoOverlayResource<P = Lhs::P>>(
    lhs: &Lhs,
    rhs: &Rhs,
    overlay_rule: OverlayRule,
) -> MultiPolygon<Lhs::P> {
    let shapes =
        FloatOverlay::with_subj_and_clip(&lhs.to_overlay_resource(), &rhs.to_overlay_resource())
            .into_graph(FILL_RULE)
            .extract_shapes(overlay_rule);
    MultiPolygon(shapes.into_iter().flat_map(paths_to_poly).collect())
}

impl<Lhs: IntoOverlayResource, Rhs: IntoOverlayResource<P = Lhs::P>> BoolOps<Rhs> for Lhs {}

// the helper traits should not be accessible by end-users of the library to prevent misuse and to
// restrict the API size
mod sealed_helper_traits {

    use super::*;

    pub trait IPoint2: Point2<S2: FloatNumber> + FloatPointCompatible<Self::S2> {}
    impl IPoint2 for Vec2 {}
    impl IPoint2 for DVec2 {}

    /// Helper trait to integrate [`i-overlay`] with `selo`.
    ///
    /// This allows us to directly use the boolops implemented in `i-overlay` with the `selo` types
    /// like [`MultiPolygon`], [`Polygon`], [`Ring`], [`Triangle`].
    ///
    /// Note: We can't use i-overlay's OverlayResource directly because their winding is the opposite from us
    ///
    /// [`i-overlay`]: https://docs.rs/i_overlay/latest/i_overlay/
    pub trait IntoOverlayResource: std::fmt::Debug {
        type P: IPoint2;
        type Resource: OverlayResource<Self::P, <Self::P as Point2>::S2> + Sized;
        fn to_overlay_resource(&self) -> Self::Resource;
    }

    impl<P: IPoint2> IntoOverlayResource for MultiPolygon<P> {
        type P = P;
        type Resource = Vec<Vec<Vec<P>>>;

        fn to_overlay_resource(&self) -> Self::Resource {
            self.iter().map(|poly| poly.to_overlay_resource()).collect()
        }
    }

    impl<P: IPoint2> IntoOverlayResource for Polygon<P> {
        type P = P;
        type Resource = Vec<Vec<P>>;

        fn to_overlay_resource(&self) -> Self::Resource {
            once(self.0.to_overlay_resource())
                .chain(self.1.iter().map(|r| r.to_overlay_resource()))
                .collect()
        }
    }

    impl<P: IPoint2> IntoOverlayResource for MultiRing<P> {
        type P = P;
        type Resource = Vec<Vec<P>>;
        fn to_overlay_resource(&self) -> Self::Resource {
            self.iter().map(|r| r.to_overlay_resource()).collect()
        }
    }

    impl<P: IPoint2> IntoOverlayResource for Ring<P> {
        type P = P;
        type Resource = Vec<P>;
        fn to_overlay_resource(&self) -> Self::Resource {
            self.0.iter().rev().cloned().collect()
        }
    }

    impl<P: IPoint2> IntoOverlayResource for Triangle<P> {
        type P = P;
        type Resource = Vec<P>;
        fn to_overlay_resource(&self) -> Self::Resource {
            self.as_ring().to_overlay_resource()
        }
    }
}

fn paths_to_poly<P: Point2>(paths: impl IntoIterator<Item = Contour<P>>) -> Option<Polygon<P>> {
    let mut paths = paths.into_iter();
    let exterior = paths.next()?;
    let interiors = paths;

    let outer = path_to_ring(exterior);
    let inner = MultiRing(interiors.map(path_to_ring).collect());
    let poly = Polygon::new(outer, inner);

    Some(poly)
}

fn path_to_ring<P: Point2>(path: Contour<P>) -> Ring<P> {
    Ring::new(
        // unfortunately, i-overlay uses opposite conventions with respect to winding compared to what
        // we have. This means, we need to flip the winding before and after using i-overlay
        path.into_iter().rev().collect::<Vec<_>>(),
    )
}

#[cfg(test)]
mod boolops_tests {

    use crate::Area;

    use super::*;

    #[test]
    fn verify_fill_rule_area_expectation() {
        // ┌─────────────────┐   ┌─────────────────┐
        // │                 │   │                 │
        // │                 │   │                 │
        // │                 │   │    ┌───────┐    │          ┌───────┐
        // │                 │   │    │       │    │          │       │
        // │                 │ - │    │       │    │  ────►   │       │
        // │                 │   │    │       │    │          │       │
        // │                 │   │    └───────┘    │          └───────┘
        // │                 │   │                 │
        // │                 │   │                 │
        // └─────────────────┘   └─────────────────┘
        let outer_ring = Ring::new(vec![
            Vec2::ZERO,
            Vec2::X * 3.0,
            Vec2::ONE * 3.0,
            Vec2::Y * 3.0,
        ]);
        let inner_ring = Ring::new(
            [Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]
                .map(|pos2| pos2 + Vec2::ONE)
                .to_vec(),
        );
        let poly_with_hole = Polygon::new(outer_ring.clone(), MultiRing(vec![inner_ring.clone()]));
        let solid_poly = Polygon::new(outer_ring, MultiRing::empty());

        let diff = solid_poly.difference(&poly_with_hole);

        assert_eq!(diff.len(), 1);
        assert_eq!(diff.area(), inner_ring.area());
    }

    #[test]
    fn verify_union_winding_expectation() {
        // ┌─────────┬─────────┐       ┌───────────────────┐
        // │         │         │       │                   │
        // │         │         │       │                   │
        // │         │         │       │                   │
        // │         │         │ ────► │                   │
        // │         │         │       │                   │
        // │         │         │       │                   │
        // │         │         │       │                   │
        // └─────────┴─────────┘       └───────────────────┘
        let ring_points = [Vec2::ZERO, Vec2::X * 0.5, Vec2::X * 0.5 + Vec2::Y, Vec2::Y];
        let ring1 = Ring::new(ring_points);
        let ring2 = Ring::new(ring_points.map(|pos2| pos2 + Vec2::X * 0.5));

        let union = ring1
            .to_polygon()
            .to_multi()
            .union(&ring2.to_polygon().to_multi());

        assert_eq!(union.len(), 1);
        assert_eq!(union.area(), 1.0);
    }

    #[test]
    fn verify_difference_winding_expectation() {
        // ┌─────────────────────┐                                    ┌─────────────────────┐
        // │                     │                                    │                     │
        // │                     │                                    │                     │
        // │                     │                                    │                     │
        // │                     │                                    │                     │
        // │                     │ - ┌──────────┐             ─────►  └──────────┐          │
        // │                     │   │          │                                │          │
        // │                     │   │          │                                │          │
        // │                     │   │          │                                │          │
        // │                     │   │          │                                │          │
        // └─────────────────────┘   └──────────┘                                └──────────┘
        let ring_points = [Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y];
        let ring1 = Ring::new(ring_points.map(|pos2| pos2 * 2.0));
        let ring2 = Ring::new(ring_points);

        let difference = ring1
            .to_polygon()
            .to_multi()
            .difference(&ring2.to_polygon().to_multi());

        assert_eq!(difference.len(), 1);
        assert_eq!(difference.area(), 3.0);
    }

    #[test]
    fn verify_intersection_winding_expectation() {
        // ┌────────┌───────────────┐                   ┌───────┐
        // │        │.......│       │                   │       │
        // │        │.......│       │                   │       │
        // │        │.......│       │                   │       │
        // │        │.......│       │                   │       │
        // │        │.......│       │   ───────────►    │       │
        // │        │.......│       │                   │       │
        // │        │.......│       │                   │       │
        // │        │.......│       │                   │       │
        // │        │.......│       │                   │       │
        // │        │.......│       │                   │       │
        // └────────└───────────────┘                   └───────┘
        let ring_points = [Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y];
        let ring1 = Ring::new(ring_points);
        let ring2 = Ring::new(ring_points.map(|pos2| pos2 + Vec2::X * 0.5));

        let intersection = ring1
            .to_polygon()
            .to_multi()
            .intersection(&ring2.to_polygon().to_multi());

        assert_eq!(intersection.len(), 1);
        assert_eq!(intersection.area(), 0.5);
    }
}
