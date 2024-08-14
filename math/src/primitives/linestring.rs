use glam::Vec2;

use super::{Line, Ring};

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

    pub fn points(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.0.iter().copied()
    }

    pub fn lines(&self) -> impl Iterator<Item = Line> + '_ {
        self.0.windows(2).map(|s| Line([s[0], s[1]]))
    }
}

// Conversions

impl From<&Ring> for LineString {
    fn from(r: &Ring) -> Self {
        r.to_linestring().into()
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

impl From<&LineString> for geo::LineString<f32> {
    fn from(r: &LineString) -> Self {
        Self(r.0.iter().map(|p| geo::Coord { x: p.x, y: p.y }).collect())
    }
}
