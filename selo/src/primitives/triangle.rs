use crate::utils::{coord_to_vec2, vec2_to_coord};

use crate::point::{Point, Point2};
use crate::{MultiRing, Ring};

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

/// A 2D Triangle
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let triangle = Triangle([Vec2::ZERO, Vec2::X, Vec2::Y]);
/// ```
#[derive(Debug, Clone, Copy, Default, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Serialize, Deserialize)
)]
pub struct Triangle<P: Point>(#[serde(bound(deserialize = ""))] pub [P; 3]);

impl<P: Point> Triangle<P> {
    /// converts the [`Triangle`] to [`Ring`]. This is mainly useful since the
    /// [`Ring`] is more general and implements more algorithms
    pub fn as_ring(self) -> Ring<P> {
        Ring::new(self.0)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Serialize, Deserialize)
)]
pub struct MultiTriangle<P: Point>(#[serde(bound(deserialize = ""))] pub Vec<Triangle<P>>);

impl<P: Point> MultiTriangle<P> {
    /// converts the [`MultiTriangle`] to [`MultiRing`]. This is mainly useful since the
    /// [`MultiRing`] is more general and implements more algorithms
    #[inline]
    pub fn as_multi_ring(&self) -> MultiRing<P> {
        MultiRing(self.0.iter().map(|tri| tri.as_ring()).collect::<Vec<_>>())
    }
}

impl<P: Point> Default for MultiTriangle<P> {
    #[inline]
    fn default() -> Self {
        Self(vec![])
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
