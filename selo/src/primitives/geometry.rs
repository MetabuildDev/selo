#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

use crate::{primitives::*, Point, SeloScalar};

/// An arbitrary flat geometry.
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let geometry = Geometry::Line(Line([Vec2::X, Vec2::Y]));
/// ```
#[derive(Debug, Clone, derive_more::From, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Serialize, Deserialize)
)]
#[serde(bound(deserialize = ""))]
pub enum Geometry<P: Point> {
    Line(Line<P>),
    LineString(LineString<P>),
    MultiLineString(MultiLineString<P>),
    Triangle(Triangle<P>),
    Ring(Ring<P>),
    MultiRing(MultiRing<P>),
    Polygon(Polygon<P>),
    MultiPolygon(MultiPolygon<P>),
}

/// A geometry that is either 2d or 3d.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Serialize, Deserialize)
)]
pub enum DynamicGeometry<S: SeloScalar> {
    Dim2(Geometry<S::Point2>),
    Dim3(Geometry<S::Point3>),
}
