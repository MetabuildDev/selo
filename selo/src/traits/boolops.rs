use std::iter::once;

use bevy_math::{DVec2, Vec2};
use i_overlay::{
    core::{fill_rule::FillRule, overlay::ShapeType, overlay_rule::OverlayRule},
    f32::overlay::F32Overlay,
    f64::overlay::F64Overlay,
    i_float::{f32_point::F32Point, f64_point::F64Point},
};

use crate::{MultiPolygon, MultiRing, Point2, Polygon, Ring, Triangle};

type BoolOpsPath<P> = Vec<<P as BoolOpsPoint>::IPoint>;
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
    Self: IntoBoolOpsPath + Sized,
    Rhs: IntoBoolOpsPath<P = <Self as IntoBoolOpsPath>::P>,
{
    type P: Point2;

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
    fn union(&self, rhs: &Rhs) -> MultiPolygon<<Self as IntoBoolOpsPath>::P> {
        <Self as IntoBoolOpsPath>::P::boolops(self, rhs, OverlayRule::Union)
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
    fn intersection(&self, rhs: &Rhs) -> MultiPolygon<<Self as IntoBoolOpsPath>::P> {
        <Self as IntoBoolOpsPath>::P::boolops(self, rhs, OverlayRule::Intersect)
    }

    /// Difference boolean operation. This creates the [`MultiPolygon`] that results from
    /// subtracting the overlap of the two input geometries from the first input geometry.
    ///
    /// ```
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
    fn difference(&self, rhs: &Rhs) -> MultiPolygon<<Self as IntoBoolOpsPath>::P> {
        <Self as IntoBoolOpsPath>::P::boolops(self, rhs, OverlayRule::Difference)
    }
}

impl<Lhs: IntoBoolOpsPath, Rhs: IntoBoolOpsPath<P = Lhs::P>> BoolOps<Rhs> for Lhs {
    type P = Lhs::P;
}

use sealed_helper_traits::*;

// the helper traits should not be accessible by end-users of the library to prevent misuse and to
// restrict the API size
mod sealed_helper_traits {
    use super::*;

    /// Helper trait to integrate [`i-overlay`] with `selo`.
    ///
    /// This allows us to directly use the boolops implemented in `i-overlay` with the `selo` types
    /// like [`MultiPolygon`], [`Polygon`], [`Ring`], [`Triangle`].
    ///
    /// [`i-overlay`]: https://docs.rs/i_overlay/latest/i_overlay/
    pub trait IntoBoolOpsPath {
        type P: BoolOpsPoint;
        fn add_paths(
            &self,
            overlay: &mut <Self::P as BoolOpsPoint>::Overlay,
            shape_type: ShapeType,
        );
    }

    impl<P: BoolOpsPoint> IntoBoolOpsPath for MultiPolygon<P> {
        type P = P;

        fn add_paths(
            &self,
            overlay: &mut <Self::P as BoolOpsPoint>::Overlay,
            shape_type: ShapeType,
        ) {
            for paths in self.iter().flat_map(poly_to_paths) {
                P::add_path(overlay, paths, shape_type);
            }
        }
    }

    impl<P: BoolOpsPoint> IntoBoolOpsPath for Polygon<P> {
        type P = P;

        fn add_paths(
            &self,
            overlay: &mut <Self::P as BoolOpsPoint>::Overlay,
            shape_type: ShapeType,
        ) {
            for paths in poly_to_paths(self) {
                P::add_path(overlay, paths, shape_type);
            }
        }
    }

    impl<P: BoolOpsPoint> IntoBoolOpsPath for Ring<P> {
        type P = P;

        fn add_paths(
            &self,
            overlay: &mut <Self::P as BoolOpsPoint>::Overlay,
            shape_type: ShapeType,
        ) {
            P::add_path(overlay, ring_to_path(self), shape_type);
        }
    }

    impl<P: BoolOpsPoint> IntoBoolOpsPath for Triangle<P> {
        type P = P;

        fn add_paths(
            &self,
            overlay: &mut <Self::P as BoolOpsPoint>::Overlay,
            shape_type: ShapeType,
        ) {
            P::add_path(overlay, ring_to_path(&self.as_ring()), shape_type);
        }
    }

    /// Helper trait to integrate [`i-overlay`] with `selo`.
    ///
    /// This allows us to implement BoolOps for different floating point types (`f32`, `f64`)
    pub trait BoolOpsPoint: Point2 {
        type IPoint: Copy;
        type Overlay;
        fn to_ipoint(self) -> Self::IPoint;
        fn from_ipoint(p: Self::IPoint) -> Self;
        fn add_path(overlay: &mut Self::Overlay, path: BoolOpsPath<Self>, shape_type: ShapeType);
        fn boolops<Lhs: IntoBoolOpsPath<P = Self>, Rhs: IntoBoolOpsPath<P = Self>>(
            lhs: &Lhs,
            rhs: &Rhs,
            overlay_rule: OverlayRule,
        ) -> MultiPolygon<Self>;
    }

    impl BoolOpsPoint for Vec2 {
        type IPoint = F32Point;
        type Overlay = F32Overlay;
        fn to_ipoint(self) -> Self::IPoint {
            F32Point::new(self.x, self.y)
        }
        fn from_ipoint(p: Self::IPoint) -> Self {
            Vec2::new(p.x, p.y)
        }

        fn add_path(overlay: &mut Self::Overlay, path: BoolOpsPath<Self>, shape_type: ShapeType) {
            overlay.add_path(path, shape_type);
        }
        fn boolops<Lhs: IntoBoolOpsPath<P = Self>, Rhs: IntoBoolOpsPath<P = Self>>(
            lhs: &Lhs,
            rhs: &Rhs,
            overlay_rule: OverlayRule,
        ) -> MultiPolygon<Self> {
            let mut overlay = F32Overlay::new();
            lhs.add_paths(&mut overlay, ShapeType::Subject);
            rhs.add_paths(&mut overlay, ShapeType::Clip);
            let graph = overlay.into_graph(FILL_RULE);
            let shapes = graph.extract_shapes(overlay_rule);
            MultiPolygon(shapes.into_iter().flat_map(paths_to_poly).collect())
        }
    }

    impl BoolOpsPoint for DVec2 {
        type IPoint = F64Point;
        type Overlay = F64Overlay;
        fn to_ipoint(self) -> Self::IPoint {
            F64Point::new(self.x, self.y)
        }
        fn from_ipoint(p: Self::IPoint) -> Self {
            DVec2::new(p.x, p.y)
        }
        fn add_path(overlay: &mut Self::Overlay, path: BoolOpsPath<Self>, shape_type: ShapeType) {
            overlay.add_path(path, shape_type);
        }
        fn boolops<Lhs: IntoBoolOpsPath<P = Self>, Rhs: IntoBoolOpsPath<P = Self>>(
            lhs: &Lhs,
            rhs: &Rhs,
            overlay_rule: OverlayRule,
        ) -> MultiPolygon<Self> {
            let mut overlay = F64Overlay::new();
            lhs.add_paths(&mut overlay, ShapeType::Subject);
            rhs.add_paths(&mut overlay, ShapeType::Clip);
            let graph = overlay.into_graph(FILL_RULE);
            let shapes = graph.extract_shapes(overlay_rule);
            MultiPolygon(shapes.into_iter().flat_map(paths_to_poly).collect())
        }
    }
}

fn poly_to_paths<P: BoolOpsPoint + Point2>(poly: &Polygon<P>) -> Vec<BoolOpsPath<P>> {
    let exterior = poly.exterior();
    let interiors = poly.interior();
    once(exterior)
        .chain(interiors.iter())
        .map(ring_to_path)
        .collect()
}

fn ring_to_path<P: BoolOpsPoint + Point2>(ring: &Ring<P>) -> BoolOpsPath<P> {
    // unfortunately, i-overlay uses opposite conventions with respect to winding compared to what
    // we have. This means, we need to flip the winding before and after using i-overlay
    ring.0.iter().rev().map(|p| p.to_ipoint()).collect()
}

fn paths_to_poly<P: BoolOpsPoint + Point2>(
    paths: impl IntoIterator<Item = Vec<P::IPoint>>,
) -> Option<Polygon<P>> {
    let mut paths = paths.into_iter();
    // just ignore poly if paths are empty
    let exterior = paths.next()?;
    let interiors = paths;

    let outer = path_to_ring(exterior);
    let inner = MultiRing(interiors.map(path_to_ring).collect());
    let poly = Polygon::new(outer, inner);

    Some(poly)
}

fn path_to_ring<P: BoolOpsPoint + Point2>(path: BoolOpsPath<P>) -> Ring<P> {
    Ring::new(
        // unfortunately, i-overlay uses opposite conventions with respect to winding compared to what
        // we have. This means, we need to flip the winding before and after using i-overlay
        path.iter()
            .rev()
            .map(|p| P::from_ipoint(*p))
            .collect::<Vec<_>>(),
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
