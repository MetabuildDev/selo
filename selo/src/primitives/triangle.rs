use crate::utils::{coord_to_vec2, vec2_to_coord};

use crate::point::{Point, Point2};

/// A 2D Triangle
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let triangle = Triangle([Vec2::ZERO, Vec2::X, Vec2::Y]);
/// ```
#[derive(Debug, Clone, Copy)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Triangle<P: Point>(pub [P; 3]);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct MultiTriangle<P: Point>(pub Vec<Triangle<P>>);

impl<P: Point> Default for MultiTriangle<P> {
    fn default() -> Self {
        Self(vec![])
    }
}

// Conversions

impl<P: Point2> From<geo::Triangle<P::S>> for Triangle<P> {
    fn from(tri: geo::Triangle<P::S>) -> Self {
        Triangle(tri.to_array().map(coord_to_vec2))
    }
}

impl<P: Point2> From<Triangle<P>> for geo::Triangle<P::S> {
    fn from(tri: Triangle<P>) -> Self {
        geo::Triangle::from(tri.0.map(vec2_to_coord))
    }
}

impl<P: Point2, TS: AsRef<[geo::Triangle<P::S>]>> From<&TS> for MultiTriangle<P> {
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

impl<P: Point2> From<&MultiTriangle<P>> for Vec<geo::Triangle<P::S>> {
    fn from(value: &MultiTriangle<P>) -> Self {
        value
            .0
            .iter()
            .copied()
            .map(|triangle| triangle.into())
            .collect::<Vec<_>>()
    }
}
