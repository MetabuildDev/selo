use std::iter::once;

use glam::Vec2;
use itertools::Itertools as _;

use super::{Line, LineString, Polygon};

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
        points.dedup();
        if points.last() == points.first() {
            points.pop();
        }
        Ring(points)
    }

    pub fn points_open(&self) -> &[Vec2] {
        &self.0
    }

    pub fn iter_points_open(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.0.iter().copied()
    }

    pub fn iter_points_closed(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.0.iter().chain(self.0.first()).copied()
    }

    pub fn to_linestring(&self) -> LineString {
        LineString(self.0.iter().cloned().chain(once(self.0[0])).collect())
    }

    pub fn to_polygon(&self) -> Polygon {
        Polygon(self.clone(), Default::default())
    }

    pub fn lines(&self) -> impl Iterator<Item = Line> + '_ {
        self.0
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| Line([*a, *b]))
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

// Conversions

impl From<geo::Triangle<f32>> for Ring {
    fn from(value: geo::Triangle<f32>) -> Self {
        Self::new(
            value
                .to_array()
                .map(|c| Vec2::new(c.x as f32, c.y as f32))
                .to_vec(),
        )
    }
}

impl TryFrom<&geo::LineString<f32>> for Ring {
    type Error = ();

    fn try_from(ls: &geo::LineString<f32>) -> Result<Self, Self::Error> {
        LineString::from(ls).to_ring().ok_or(())
    }
}

impl From<&Ring> for geo::LineString<f32> {
    fn from(value: &Ring) -> Self {
        (&value.to_linestring()).into()
    }
}
