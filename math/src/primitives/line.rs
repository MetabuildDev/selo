use glam::Vec2;

use crate::utils::{coord_to_vec2, vec2_to_coord};

/// A 2D Line
///
/// # Example
///
/// ```
/// # use math::Line;
/// # use glam::Vec2;
///
/// let line = Line([Vec2::X, Vec2::Y]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Line(pub [Vec2; 2]);

// Conversions

impl From<geo::Line<f32>> for Line {
    fn from(ls: geo::Line<f32>) -> Self {
        Line([coord_to_vec2(ls.start), coord_to_vec2(ls.end)])
    }
}

impl From<Line> for geo::Line<f32> {
    fn from(line: Line) -> Self {
        geo::Line::new(vec2_to_coord(line.0[0]), vec2_to_coord(line.0[1]))
    }
}
