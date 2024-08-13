use std::{fmt::Debug, iter::once};

use bevy_math::Vec2;
#[cfg(feature = "bevy")]
use {bevy_ecs::prelude::Component, bevy_reflect::Reflect};

#[cfg_attr(feature = "bevy", derive(Reflect))]
#[derive(Debug, Clone, Copy)]
pub struct Line(pub [Vec2; 2]);

#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Triangle(pub [Vec2; 3]);

/// Represents the set of points in the lines represented by each consecutive pair of points
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct LineString(pub Vec<Vec2>);

impl Default for LineString {
    fn default() -> Self {
        Self(vec![])
    }
}

impl LineString {
    pub fn close(self) -> Ring {
        Ring::new(self.0)
    }

    /// If this is a closed linestring, this will give the
    pub fn inside(&self) -> Option<Ring> {
        (self.0.last() == self.0.first()).then(|| Ring::new(self.0.clone()))
    }
}

/// Represents the inside area of a closed LineString.
/// The first coordinate is different from the last, the line connecting them is implied.
#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Ring(Vec<Vec2>);
impl Default for Ring {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Ring {
    pub fn new(mut points: Vec<Vec2>) -> Self {
        if points.last() == points.first() {
            points.pop();
        }
        Ring(points)
    }

    pub fn points(&self) -> &[Vec2] {
        &self.0
    }

    pub fn to_linestring(&self) -> LineString {
        LineString(self.0.iter().cloned().chain(once(self.0[0])).collect())
    }

    pub fn to_polygon(&self) -> Polygon {
        Polygon(self.clone(), Default::default())
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct MultiRing(pub Vec<Ring>);
impl Default for MultiRing {
    fn default() -> Self {
        Self(vec![])
    }
}

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Polygon(pub Ring, pub MultiRing);

impl Polygon {
    pub fn new(exterior: Ring, interior: MultiRing) -> Self {
        Self(exterior, interior)
    }

    pub fn exterior(&self) -> &Ring {
        &self.0
    }
    pub fn interior(&self) -> &MultiRing {
        &self.1
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct MultiPolygon(pub Vec<Polygon>);
impl Default for MultiPolygon {
    fn default() -> Self {
        Self(vec![])
    }
}
