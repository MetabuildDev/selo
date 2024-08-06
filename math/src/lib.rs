use geo::*;
use glam::*;
use line_intersection::line_intersection;

fn coord_to_vec2(coord: Coord<f32>) -> Vec2 {
    Vec2::new(coord.x, coord.y)
}

fn vec2_to_coord(vec2: Vec2) -> Coord<f32> {
    Coord {
        x: vec2.x,
        y: vec2.y,
    }
}

fn vec2s_to_polygon(polygon: impl IntoIterator<Item = Vec2>) -> geo::Polygon<f32> {
    Polygon::<f32>::new(
        LineString::<f32>::new(polygon.into_iter().map(vec2_to_coord).collect::<Vec<_>>()),
        vec![],
    )
}

fn linestring_to_vec2s(ls: &LineString<f32>) -> impl Iterator<Item = Vec2> + '_ {
    ls.0.iter().copied().map(coord_to_vec2)
}

fn coord_up_precision(c: Coord<f32>) -> Coord<f64> {
    Coord {
        x: c.x as f64,
        y: c.y as f64,
    }
}

fn coord_down_precision(c: Coord<f64>) -> Coord<f32> {
    Coord {
        x: c.x as f32,
        y: c.y as f32,
    }
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
