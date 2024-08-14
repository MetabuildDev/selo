use glam::Vec2;

use crate::utils::{coord_to_vec2, vec2_to_coord};

#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Triangle(pub [Vec2; 3]);

// Geo conversions

impl From<geo::Triangle<f32>> for Triangle {
    fn from(tri: geo::Triangle<f32>) -> Self {
        Triangle(tri.to_array().map(coord_to_vec2))
    }
}

impl From<Triangle> for geo::Triangle<f32> {
    fn from(tri: Triangle) -> Self {
        geo::Triangle::from(tri.0.map(vec2_to_coord))
    }
}
