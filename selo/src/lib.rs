#![allow(refining_impl_trait)]

pub use embedded_primitive::*;
use geo::{MapCoords as _, SpadeBoolops, StitchTriangles as _, TriangulateSpade as _};

mod errors;

mod embedded_primitive;
mod workplane;

mod traits;
pub use traits::*;

mod utils;
use utils::*;

pub mod primitives;
pub use primitives::*;

mod point;
pub use point::*;

mod algorithms;

#[cfg(feature = "wkt")]
pub mod wkt;

use glam::*;

pub mod prelude {
    pub use super::algorithms::*;
    pub use super::embedded_primitive::{Embed, FlatPrimitive, Unembed};
    pub use super::errors::GeometryError;
    pub use super::point::*;
    pub use super::primitives::*;
    pub use super::traits::*;
    pub use super::workplane::Workplane;
    pub use glam::*;
}

pub fn intersect_line_2d_point<P: Point2>(a: Line<P>, b: Line<P>) -> Option<P> {
    geo::line_intersection::line_intersection(a.into(), b.into()).and_then(|coord| match coord {
        geo::LineIntersection::SinglePoint {
            intersection,
            is_proper,
        } => is_proper.then_some(coord_to_vec2(intersection)),
        geo::LineIntersection::Collinear { intersection: _ } => None,
    })
}

pub fn triangulate_glam<P: Point2>(polygon: Polygon<P>) -> Vec<Triangle<P>> {
    let triangles = geo::Polygon::<P::S>::from(&polygon)
        .constrained_triangulation(geo::triangulate_spade::SpadeTriangulationConfig {
            snap_radius: P::S::from(0.001),
        })
        .unwrap();

    triangles
        .into_iter()
        .map(Triangle::from)
        .collect::<Vec<_>>()
}

pub fn stitch_triangles_glam<P: Point2>(
    triangles: impl IntoIterator<Item = Triangle<P>>,
) -> Vec<Ring<P>> {
    let geo_triangles = triangles
        .into_iter()
        .map(geo::Triangle::from)
        .collect::<Vec<_>>();

    let polys = geo_triangles
        .stitch_triangulation()
        .map(|mp| mp.0)
        .unwrap_or_default();

    polys
        .into_iter()
        .map(|poly| Ring::try_from(poly.exterior()).unwrap())
        .collect::<Vec<_>>()
}

pub fn boolops_union_glam<P: Point2>(rings: impl IntoIterator<Item = Ring<P>>) -> MultiPolygon<P> {
    let rings = rings.into_iter().collect::<Vec<_>>();

    rings
        .clone()
        .into_iter()
        .map(|ring| ring.to_polygon().to_multi().to_geo())
        .try_fold(empty_multipolygon::<P>(), |multi_poly, other| {
            SpadeBoolops::union(&multi_poly, &other)
        })
        .map(|multi_poly| multi_poly.to_selo())
        .unwrap_or(MultiPolygon(
            rings.into_iter().map(|ring| ring.to_polygon()).collect(),
        ))
}

pub fn buffer_polygon_glam<P: Point2>(polygon: &Polygon<P>, expand_by: f64) -> MultiPolygon<P> {
    let geo_polygon = geo::Polygon::<P::S>::from(polygon);
    let polygon_f64 = geo_polygon.map_coords(cast_coord);

    let buffered = geo_buffer::buffer_polygon(&polygon_f64, expand_by);

    let buffered_f32 = buffered.map_coords(cast_coord);

    (&buffered_f32).into()
}

pub fn skeleton_lines_glam<P: Point2>(
    polygon: Polygon<P>,
    orientation: bool,
) -> Vec<LineString<P>> {
    let geo_polygon = geo::Polygon::<P::S>::from(&polygon);
    let polygon_f64 = geo_polygon.map_coords(cast_coord);

    let skeleton_lines = geo_buffer::skeleton_of_polygon_to_linestring(&polygon_f64, orientation);

    skeleton_lines
        .into_iter()
        .map(|ls| ls.map_coords(cast_coord))
        .map(|ls| LineString::<P>::from(&ls))
        .collect::<Vec<_>>()
}
