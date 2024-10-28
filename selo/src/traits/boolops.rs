use std::iter::once;

use bevy_math::{DVec2, Vec2};
use i_overlay::{
    core::{fill_rule::FillRule, overlay::ShapeType, overlay_rule::OverlayRule},
    f32::overlay::F32Overlay,
    f64::overlay::F64Overlay,
    i_float::{f32_point::F32Point, f64_point::F64Point},
};

use crate::{MultiPolygon, MultiRing, Point2, Polygon, Ring};

type BoolOpsPath<P> = Vec<<P as IntoIOverlayPoint>::IPoint>;

pub trait BoolOps {
    type P: Point2;

    fn boolop(&self, rhs: &Self, overlay_rule: OverlayRule) -> MultiPolygon<Self::P>;

    fn union(&self, rhs: &Self) -> MultiPolygon<Self::P> {
        self.boolop(rhs, OverlayRule::Union)
    }
    fn intersection(&self, rhs: &Self) -> MultiPolygon<Self::P> {
        self.boolop(rhs, OverlayRule::Intersect)
    }
    fn difference(&self, rhs: &Self) -> MultiPolygon<Self::P> {
        self.boolop(rhs, OverlayRule::Difference)
    }
}

impl BoolOps for MultiPolygon<Vec2> {
    type P = Vec2;

    fn boolop(&self, rhs: &Self, overlay_rule: OverlayRule) -> MultiPolygon<Self::P> {
        let mut overlay = F32Overlay::new();
        for a in self.iter().map(poly_to_paths) {
            overlay.add_paths(a, ShapeType::Subject);
        }
        for b in rhs.iter().map(poly_to_paths) {
            overlay.add_paths(b, ShapeType::Clip);
        }
        let graph = overlay.into_graph(FillRule::EvenOdd);
        let shapes = graph.extract_shapes(overlay_rule);
        MultiPolygon(shapes.into_iter().flat_map(paths_to_poly).collect())
    }
}

impl BoolOps for MultiPolygon<DVec2> {
    type P = DVec2;

    fn boolop(&self, rhs: &Self, overlay_rule: OverlayRule) -> MultiPolygon<Self::P> {
        let mut overlay = F64Overlay::new();
        for a in self.iter().map(poly_to_paths) {
            overlay.add_paths(a, ShapeType::Subject);
        }
        for b in rhs.iter().map(poly_to_paths) {
            overlay.add_paths(b, ShapeType::Clip);
        }
        let graph = overlay.into_graph(FillRule::EvenOdd);
        let shapes = graph.extract_shapes(overlay_rule);
        MultiPolygon(shapes.into_iter().flat_map(paths_to_poly).collect())
    }
}

trait IntoIOverlayPoint {
    type IPoint: Copy;
    type Overlay;
    fn to_ipoint(self) -> Self::IPoint;
    fn from_ipoint(p: Self::IPoint) -> Self;
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

        let diff = solid_poly.to_multi().difference(&poly_with_hole.to_multi());

        assert_eq!(inner_ring.area(), diff.area());
    }
}
