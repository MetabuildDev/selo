mod utils;
mod working_plane;

use geo::*;
use glam::*;
use line_intersection::line_intersection;

use utils::*;

pub mod prelude {
    pub use super::working_plane::WorkingPlane;
}

pub fn intersect_line_2d_point(
    (start1, end1): (Vec2, Vec2),
    (start2, end2): (Vec2, Vec2),
) -> Option<Vec2> {
    let line1 = Line::new(vec2_to_coord(start1), vec2_to_coord(end1));
    let line2 = Line::new(vec2_to_coord(start2), vec2_to_coord(end2));

    line_intersection(line1, line2).and_then(|coord| match coord {
        LineIntersection::SinglePoint {
            intersection,
            is_proper,
        } => is_proper.then_some(coord_to_vec2(intersection)),
        LineIntersection::Collinear { intersection: _ } => None,
    })
}

pub fn triangulate_glam(polygon: impl IntoIterator<Item = Vec2>) -> Vec<[Vec2; 3]> {
    let geo_polygon = vec2s_to_polygon(polygon);

    let triangles = geo_polygon
        .constrained_triangulation(triangulate_spade::SpadeTriangulationConfig {
            snap_radius: 0.001,
        })
        .unwrap();

    triangles
        .into_iter()
        .map(|tri| [tri.0, tri.1, tri.2].map(coord_to_vec2))
        .collect::<Vec<_>>()
}

pub fn stitch_triangles_glam(triangles: impl IntoIterator<Item = [Vec2; 3]>) -> Vec<Vec<Vec2>> {
    let geo_triangles = vec2s_to_triangles(triangles);

    let polys = geo_triangles
        .stitch_triangulation()
        .map(|mp| mp.0)
        .unwrap_or_default();

    polys
        .into_iter()
        .map(|poly| linestring_to_vec2s(poly.exterior()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn boolops_union_glam(polygons: impl IntoIterator<Item = Vec<Vec2>>) -> Vec<Vec<Vec2>> {
    let inputs = polygons.into_iter().collect::<Vec<_>>();

    inputs
        .clone()
        .into_iter()
        .map(vec2s_to_polygon)
        .map(|poly| MultiPolygon::new(vec![poly]))
        .try_fold(empty_multipolygon(), |poly, other| {
            SpadeBoolops::union(&poly, &other)
        })
        .map(|multi_poly| {
            multi_poly
                .into_iter()
                .map(|poly| linestring_to_vec2s(poly.exterior()).collect::<Vec<_>>())
                .collect::<Vec<_>>()
        })
        .unwrap_or(inputs)
}

pub fn buffer_polygon_glam(
    polygon: impl IntoIterator<Item = Vec2>,
    expand_by: f64,
) -> Vec<Vec<Vec2>> {
    let geo_polygon = vec2s_to_polygon(polygon);
    let polygon_f64 = geo_polygon.map_coords(coord_up_precision);

    let buffered = geo_buffer::buffer_polygon(&polygon_f64, expand_by);

    let buffered_f32 = buffered.map_coords(coord_down_precision);

    buffered_f32
        .into_iter()
        .map(|poly| linestring_to_vec2s(poly.exterior()).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}

pub fn skeleton_lines_glam(
    polygon: impl IntoIterator<Item = Vec2>,
    orientation: bool,
) -> Vec<Vec<Vec2>> {
    let geo_polygon = vec2s_to_polygon(polygon);
    let polygon_f64 = geo_polygon.map_coords(coord_up_precision);

    let skeleton_lines = geo_buffer::skeleton_of_polygon_to_linestring(&polygon_f64, orientation);

    skeleton_lines
        .into_iter()
        .map(|ls| ls.map_coords(coord_down_precision))
        .map(|ls| linestring_to_vec2s(&ls).collect::<Vec<_>>())
        .collect::<Vec<_>>()
}
