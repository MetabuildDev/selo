use crate::utils::{coord_to_vec2, vec2_to_coord};

use super::{Point, Point2};

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
pub struct Line<P: Point>(pub [P; 2]);

// Conversions

impl<P: Point2> From<geo::Line<P::Float>> for Line<P> {
    fn from(ls: geo::Line<P::Float>) -> Self {
        Line([coord_to_vec2(ls.start), coord_to_vec2(ls.end)])
    }
}

impl<P: Point2> From<Line<P>> for geo::Line<P::Float> {
    fn from(line: Line<P>) -> Self {
        geo::Line::new(vec2_to_coord(line.0[0]), vec2_to_coord(line.0[1]))
    }
}
