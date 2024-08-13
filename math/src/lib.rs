mod conversions;
pub mod primitives;
mod utils;
mod working_plane;

use conversions::coord_to_vec2;
use geo::{MapCoords as _, StitchTriangles as _, TriangulateSpade as _};
use primitives::*;

// use geo::*;
use glam::*;
// use line_intersection::line_intersection;

use utils::*;

pub mod prelude {
    pub use super::working_plane::WorkingPlane;
}

pub fn intersect_line_2d_point(a: Line, b: Line) -> Option<Vec2> {
    geo::line_intersection::line_intersection(a.into(), b.into()).and_then(|coord| match coord {
        geo::LineIntersection::SinglePoint {
            intersection,
            is_proper,
        } => is_proper.then_some(coord_to_vec2(intersection)),
        geo::LineIntersection::Collinear { intersection: _ } => None,
    })
}

pub fn triangulate_glam(polygon: Polygon) -> Vec<Triangle> {
    let triangles = geo::Polygon::from(&polygon)
        .constrained_triangulation(geo::triangulate_spade::SpadeTriangulationConfig {
            snap_radius: 0.001,
        })
        .unwrap();

    triangles
        .into_iter()
        .map(Triangle::from)
        .collect::<Vec<_>>()
}

pub fn stitch_triangles_glam(triangles: impl IntoIterator<Item = Triangle>) -> Vec<Ring> {
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

pub fn buffer_polygon_glam(polygon: Polygon, expand_by: f64) -> Vec<Ring> {
    let geo_polygon = geo::Polygon::from(&polygon);
    let polygon_f64 = geo_polygon.map_coords(coord_up_precision);

    let buffered = geo_buffer::buffer_polygon(&polygon_f64, expand_by);

    let buffered_f32 = buffered.map_coords(coord_down_precision);

    buffered_f32
        .into_iter()
        .map(|poly| Ring::try_from(poly.exterior()).unwrap())
        .collect::<Vec<_>>()
}

pub fn skeleton_lines_glam(polygon: Polygon, orientation: bool) -> Vec<LineString> {
    let geo_polygon = geo::Polygon::from(&polygon);
    let polygon_f64 = geo_polygon.map_coords(coord_up_precision);

    let skeleton_lines = geo_buffer::skeleton_of_polygon_to_linestring(&polygon_f64, orientation);

    skeleton_lines
        .into_iter()
        .map(|ls| ls.map_coords(coord_down_precision))
        .map(|ls| LineString::from(&ls))
        .collect::<Vec<_>>()
}
