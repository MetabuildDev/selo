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
