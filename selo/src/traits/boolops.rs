use std::iter::once;

use bevy_math::{DVec2, Vec2};
use i_overlay::{
    core::{fill_rule::FillRule, overlay::ShapeType, overlay_rule::OverlayRule},
    f32::overlay::F32Overlay,
    f64::overlay::F64Overlay,
    i_float::{f32_point::F32Point, f64_point::F64Point},
};

use crate::{MultiPolygon, MultiRing, Point2, Polygon, Ring};

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

fn poly_to_paths<P: IntoIOverlayPoint + Point2>(poly: &Polygon<P>) -> Vec<Vec<P::IPoint>> {
    once(ring_to_path(poly.exterior()))
        .chain(poly.interior().iter().map(ring_to_path))
        .collect()
}

fn ring_to_path<P: IntoIOverlayPoint + Point2>(ring: &Ring<P>) -> Vec<P::IPoint> {
    ring.0.iter().map(|p| p.to_ipoint()).collect()
}

fn paths_to_poly<P: IntoIOverlayPoint + Point2>(paths: Vec<Vec<P::IPoint>>) -> Option<Polygon<P>> {
    let mut iter = paths.into_iter();
    let exterior = iter.next()?;
    let interiors = iter;
    Some(Polygon::new(
        path_to_ring(exterior),
        MultiRing(interiors.map(path_to_ring).collect()),
    ))
}

fn path_to_ring<P: IntoIOverlayPoint + Point2>(path: Vec<P::IPoint>) -> Ring<P> {
    Ring::new(path.iter().map(|p| P::from_ipoint(*p)).collect::<Vec<_>>())
}
