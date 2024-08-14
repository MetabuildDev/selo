use geo::*;
use glam::*;

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

pub(crate) fn coord_to_vec2(coord: geo::Coord<f32>) -> Vec2 {
    Vec2::new(coord.x, coord.y)
}

pub(crate) fn vec2_to_coord(vec2: Vec2) -> geo::Coord<f32> {
    geo::Coord {
        x: vec2.x,
        y: vec2.y,
    }
}
