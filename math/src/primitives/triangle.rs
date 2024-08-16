use glam::Vec2;

use crate::utils::{coord_to_vec2, vec2_to_coord};

/// A 2D Triangle
///
/// # Example
///
/// ```
/// # use math::Triangle;
/// # use glam::Vec2;
///
/// let triangle = Triangle([Vec2::ZERO, Vec2::X, Vec2::Y]);
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Triangle(pub [Vec2; 3]);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct MultiTriangle(pub Vec<Triangle>);

impl Default for MultiTriangle {
    fn default() -> Self {
        Self(vec![])
    }
}

// Conversions

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

impl<TS: AsRef<[geo::Triangle<f32>]>> From<&TS> for MultiTriangle {
    fn from(value: &TS) -> Self {
        MultiTriangle(
            value
                .as_ref()
                .iter()
                .copied()
                .map(|triangle| triangle.into())
                .collect(),
        )
    }
}

impl From<&MultiTriangle> for Vec<geo::Triangle<f32>> {
    fn from(value: &MultiTriangle) -> Self {
        value
            .0
            .iter()
            .copied()
            .map(|triangle| triangle.into())
            .collect::<Vec<_>>()
    }
}
