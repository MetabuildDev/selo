use std::iter::once;

use bevy_math::prelude::*;

use super::primitives::{Polygon, *};

pub(crate) fn coord_to_vec2(coord: geo::Coord<f32>) -> Vec2 {
    Vec2::new(coord.x, coord.y)
}

pub(crate) fn vec2_to_coord(vec2: Vec2) -> geo::Coord<f32> {
    geo::Coord {
        x: vec2.x,
        y: vec2.y,
    }
}

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

impl From<&geo::LineString<f32>> for LineString {
    fn from(ls: &geo::LineString<f32>) -> Self {
        LineString(
            ls.0.iter()
                .map(|c| Vec2::new(c.x as f32, c.y as f32))
                .collect(),
        )
    }
}

impl TryFrom<&geo::LineString<f32>> for Ring {
    type Error = ();

    fn try_from(ls: &geo::LineString<f32>) -> Result<Self, Self::Error> {
        if ls.is_closed() {
            Ok(Ring::new(
                ls.0[..ls.0.len() - 1]
                    .iter()
                    .map(|c| Vec2::new(c.x as f32, c.y as f32))
                    .collect(),
            ))
        } else {
            Err(())
        }
    }
}

impl From<&LineString> for geo::LineString<f32> {
    fn from(r: &LineString) -> Self {
        Self(r.0.iter().map(|p| geo::Coord { x: p.x, y: p.y }).collect())
    }
}

impl From<&Ring> for LineString {
    fn from(r: &Ring) -> Self {
        r.to_linestring().into()
    }
}

impl From<&geo::Triangle<f32>> for Ring {
    fn from(value: &geo::Triangle<f32>) -> Self {
        Self::new(
            value
                .to_array()
                .map(|c| Vec2::new(c.x as f32, c.y as f32))
                .to_vec(),
        )
    }
}

impl From<&Polygon> for geo::Polygon<f32> {
    fn from(value: &Polygon) -> Self {
        geo::Polygon::new(
            value.exterior().into(),
            value.interior().0.iter().map(|r| r.into()).collect(),
        )
    }
}

impl From<&geo::Polygon<f32>> for Polygon {
    fn from(value: &geo::Polygon<f32>) -> Self {
        Polygon(
            Ring::try_from(value.exterior()).unwrap(),
            MultiRing(
                value
                    .interiors()
                    .iter()
                    .map(|r| Ring::try_from(r).unwrap())
                    .collect(),
            ),
        )
    }
}

impl From<&Ring> for geo::LineString<f32> {
    fn from(value: &Ring) -> Self {
        let first = value.points()[0];
        geo::LineString::new(
            value
                .points()
                .iter()
                .cloned()
                .chain(once(first))
                .map(|p| geo::Coord { x: p.x, y: p.y })
                .collect(),
        )
    }
}
