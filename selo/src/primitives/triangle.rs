use itertools::Itertools;

use crate::utils::{coord_to_vec2, vec2_to_coord};

use crate::point::{Point, Point2};
use crate::{Area, SeloScalar, ToGeo, ToSelo, Wedge};

/// A 2D Triangle
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let triangle = Triangle([Vec2::ZERO, Vec2::X, Vec2::Y]);
/// ```
#[derive(Debug, Clone, Copy, Default)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Triangle<P: Point>(pub [P; 3]);

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct MultiTriangle<P: Point>(pub Vec<Triangle<P>>);

impl<P: Point> Default for MultiTriangle<P> {
    #[inline]
    fn default() -> Self {
        Self(vec![])
    }
}

// Traits

impl<P: Point> Area for Triangle<P> {
    type P = P;

    #[inline]
    fn area(&self) -> <P as Wedge>::Output {
        self.0
            .into_iter()
            .circular_tuple_windows()
            .map(|(a, b)| a.wedge(b))
            .sum::<<P as Wedge>::Output>()
            / <<P as Point>::S as From<f32>>::from(2f32)
    }
}

// Conversions

impl<P: Point2> From<geo::Triangle<P::S>> for Triangle<P> {
    #[inline]
    fn from(tri: geo::Triangle<P::S>) -> Self {
        Triangle(tri.to_array().map(coord_to_vec2))
    }
}

impl<P: Point2> From<Triangle<P>> for geo::Triangle<P::S> {
    #[inline]
    fn from(tri: Triangle<P>) -> Self {
        geo::Triangle::from(tri.0.map(vec2_to_coord))
    }
}

impl<P: Point2, TS: AsRef<[geo::Triangle<P::S>]>> From<&TS> for MultiTriangle<P> {
    #[inline]
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
    #[inline]
    fn from(value: &MultiTriangle<P>) -> Self {
        value
            .0
            .iter()
            .copied()
            .map(|triangle| triangle.into())
            .collect::<Vec<_>>()
    }
}

impl<P: Point2> ToGeo for Triangle<P> {
    type GeoType = geo::Triangle<P::S>;

    #[inline]
    fn to_geo(self) -> Self::GeoType {
        self.into()
    }
}

impl<S: SeloScalar> ToSelo for geo::Triangle<S> {
    type SeloType = Triangle<S::Point2>;

    #[inline]
    fn to_selo(self) -> Self::SeloType {
        self.into()
    }
}
