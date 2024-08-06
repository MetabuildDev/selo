use geo::*;
use glam::*;

pub(crate) fn coord_to_vec2(coord: Coord<f32>) -> Vec2 {
    Vec2::new(coord.x, coord.y)
}

pub(crate) fn vec2_to_coord(vec2: Vec2) -> Coord<f32> {
    Coord {
        x: vec2.x,
        y: vec2.y,
    }
}

pub(crate) fn vec2s_to_polygon(polygon: impl IntoIterator<Item = Vec2>) -> geo::Polygon<f32> {
    Polygon::<f32>::new(
        LineString::<f32>::new(polygon.into_iter().map(vec2_to_coord).collect::<Vec<_>>()),
        vec![],
    )
}

pub(crate) fn linestring_to_vec2s(ls: &LineString<f32>) -> impl Iterator<Item = Vec2> + '_ {
    ls.0.iter().copied().map(coord_to_vec2)
}

pub(crate) fn coord_up_precision(c: Coord<f32>) -> Coord<f64> {
    Coord {
        x: c.x as f64,
        y: c.y as f64,
    }
}

pub(crate) fn coord_down_precision(c: Coord<f64>) -> Coord<f32> {
    Coord {
        x: c.x as f32,
        y: c.y as f32,
    }
}
