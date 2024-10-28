use std::iter::once;

use bevy_math::{DVec2, Vec2};
use i_overlay::{
    core::{fill_rule::FillRule, overlay::ShapeType, overlay_rule::OverlayRule},
    f32::overlay::F32Overlay,
    f64::overlay::F64Overlay,
    i_float::{f32_point::F32Point, f64_point::F64Point},
};

use crate::{MultiPolygon, MultiRing, Point2, Polygon, Ring, Triangle};

type BoolOpsPath<P> = Vec<<P as IntoIOverlayPoint>::IPoint>;

pub trait BoolOps<Rhs> {
    type P: Point2;

    fn boolop(&self, rhs: &Rhs, overlay_rule: OverlayRule) -> MultiPolygon<Self::P>;

    fn union(&self, rhs: &Rhs) -> MultiPolygon<Self::P> {
        self.boolop(rhs, OverlayRule::Union)
    }
    fn intersection(&self, rhs: &Rhs) -> MultiPolygon<Self::P> {
        self.boolop(rhs, OverlayRule::Intersect)
    }
    fn difference(&self, rhs: &Rhs) -> MultiPolygon<Self::P> {
        self.boolop(rhs, OverlayRule::Difference)
    }
}

impl<Lhs: IntoIOverlayPaths, Rhs: IntoIOverlayPaths<IOverlayPoint = Lhs::IOverlayPoint>>
    BoolOps<Rhs> for Lhs
{
    type P = Lhs::IOverlayPoint;

    fn boolop(&self, rhs: &Rhs, overlay_rule: OverlayRule) -> MultiPolygon<Self::P> {
        Self::P::boolops(self, rhs, overlay_rule)
    }
}

pub trait IntoIOverlayPaths {
    type IOverlayPoint: IntoIOverlayPoint;
    fn add_paths(
        &self,
        overlay: &mut <Self::IOverlayPoint as IntoIOverlayPoint>::Overlay,
        shape_type: ShapeType,
    );
}

impl<P: IntoIOverlayPoint> IntoIOverlayPaths for MultiPolygon<P> {
    type IOverlayPoint = P;

    fn add_paths(
        &self,
        overlay: &mut <Self::IOverlayPoint as IntoIOverlayPoint>::Overlay,
        shape_type: ShapeType,
    ) {
        for paths in self.iter().flat_map(poly_to_paths) {
            P::add_path(overlay, paths, shape_type);
        }
    }
}

impl<P: IntoIOverlayPoint> IntoIOverlayPaths for Polygon<P> {
    type IOverlayPoint = P;

    fn add_paths(
        &self,
        overlay: &mut <Self::IOverlayPoint as IntoIOverlayPoint>::Overlay,
        shape_type: ShapeType,
    ) {
        for paths in poly_to_paths(self) {
            P::add_path(overlay, paths, shape_type);
        }
    }
}

impl<P: IntoIOverlayPoint> IntoIOverlayPaths for Ring<P> {
    type IOverlayPoint = P;

    fn add_paths(
        &self,
        overlay: &mut <Self::IOverlayPoint as IntoIOverlayPoint>::Overlay,
        shape_type: ShapeType,
    ) {
        P::add_path(overlay, ring_to_path(self), shape_type);
    }
}

impl<P: IntoIOverlayPoint> IntoIOverlayPaths for Triangle<P> {
    type IOverlayPoint = P;

    fn add_paths(
        &self,
        overlay: &mut <Self::IOverlayPoint as IntoIOverlayPoint>::Overlay,
        shape_type: ShapeType,
    ) {
        P::add_path(overlay, ring_to_path(&self.as_ring()), shape_type);
    }
}

pub trait IntoIOverlayPoint: Point2 {
    type IPoint: Copy;
    type Overlay;
    fn to_ipoint(self) -> Self::IPoint;
    fn from_ipoint(p: Self::IPoint) -> Self;
    fn add_path(overlay: &mut Self::Overlay, path: BoolOpsPath<Self>, shape_type: ShapeType);
    fn boolops<
        Lhs: IntoIOverlayPaths<IOverlayPoint = Self>,
        Rhs: IntoIOverlayPaths<IOverlayPoint = Self>,
    >(
        lhs: &Lhs,
        rhs: &Rhs,
        overlay_rule: OverlayRule,
    ) -> MultiPolygon<Self>;
}

impl IntoIOverlayPoint for Vec2 {
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
    fn boolops<
        Lhs: IntoIOverlayPaths<IOverlayPoint = Self>,
        Rhs: IntoIOverlayPaths<IOverlayPoint = Self>,
    >(
        lhs: &Lhs,
        rhs: &Rhs,
        overlay_rule: OverlayRule,
    ) -> MultiPolygon<Self> {
        let mut overlay = F32Overlay::new();
        lhs.add_paths(&mut overlay, ShapeType::Subject);
        rhs.add_paths(&mut overlay, ShapeType::Clip);
        let graph = overlay.into_graph(FillRule::EvenOdd);
        let shapes = graph.extract_shapes(overlay_rule);
        MultiPolygon(shapes.into_iter().flat_map(paths_to_poly).collect())
    }
}

impl IntoIOverlayPoint for DVec2 {
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
    fn boolops<
        Lhs: IntoIOverlayPaths<IOverlayPoint = Self>,
        Rhs: IntoIOverlayPaths<IOverlayPoint = Self>,
    >(
        lhs: &Lhs,
        rhs: &Rhs,
        overlay_rule: OverlayRule,
    ) -> MultiPolygon<Self> {
        let mut overlay = F64Overlay::new();
        lhs.add_paths(&mut overlay, ShapeType::Subject);
        rhs.add_paths(&mut overlay, ShapeType::Clip);
        let graph = overlay.into_graph(FillRule::EvenOdd);
        let shapes = graph.extract_shapes(overlay_rule);
        MultiPolygon(shapes.into_iter().flat_map(paths_to_poly).collect())
    }
}

fn poly_to_paths<P: IntoIOverlayPoint + Point2>(poly: &Polygon<P>) -> Vec<BoolOpsPath<P>> {
    let exterior = poly.exterior();
    let interiors = poly.interior();
    once(exterior)
        .chain(interiors.iter())
        .map(ring_to_path)
        .collect()
}

fn ring_to_path<P: IntoIOverlayPoint + Point2>(ring: &Ring<P>) -> BoolOpsPath<P> {
    ring.0.iter().rev().map(|p| p.to_ipoint()).collect()
}

fn paths_to_poly<P: IntoIOverlayPoint + Point2>(
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

fn path_to_ring<P: IntoIOverlayPoint + Point2>(path: BoolOpsPath<P>) -> Ring<P> {
    Ring::new(
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
    fn verify_fill_rule_invariant_expectation() {
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

        assert_eq!(inner_ring.area(), diff.area());
    }
}
