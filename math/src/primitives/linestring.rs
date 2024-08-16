use glam::Vec2;

use super::{Line, Ring};

/// Represents the set of points in the lines represented by each consecutive pair of points.
///
/// There's no connecting [`Line`] between the first and the last point.
///
/// # Example
///
/// ```
/// # use math::LineString;
/// # use glam::Vec2;
///
/// let line = LineString(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct LineString(pub Vec<Vec2>);

impl Default for LineString {
    fn default() -> Self {
        Self(vec![])
    }
}

impl LineString {
    /// constructs an empty [`LineString`]
    ///
    /// # Example
    ///
    /// ```
    /// # use math::LineString;
    ///
    /// let empty = LineString::empty();
    ///
    /// assert!(empty.0.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a new [`LineString`] enforcing its invariants if necessary.
    ///
    /// # Example
    ///
    /// ```
    /// # use math::LineString;
    /// # use glam::Vec2;
    ///
    /// let linestring = LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// // repetitive points are messing with most of the algorithms in 2D, so the constructor
    /// // deals with this itself and fixes it internally. Inputs like these happen on accident
    /// // most of the times anyways
    /// let linestring_deduped = LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::X, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let expected = vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y];
    ///
    /// assert_eq!(&linestring.0, &expected);
    /// assert_eq!(&linestring_deduped.0, &expected);
    /// ```
    pub fn new(mut points: Vec<Vec2>) -> Self {
        points.dedup();
        LineString(points)
    }

    /// Returns whether the [`LineString`] is defining a closed shape, meaning the first and last
    /// point coincide.
    ///
    /// # Example
    ///
    /// ```
    /// # use math::LineString;
    /// # use glam::Vec2;
    ///
    /// let open_linestring = LineString::new(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
    /// let closed_linestring = LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::Y, Vec2::ZERO]);
    ///
    /// assert!(!open_linestring.closed());
    /// assert!(closed_linestring.closed());
    /// ```
    pub fn closed(&self) -> bool {
        self.0.last() == self.0.first()
    }

    /// If this is a [`LineString`] is closed, turn it into a [`Ring`]. Otherwise returns `None`
    ///
    /// # Example
    ///
    /// ```
    /// # use math::LineString;
    /// # use glam::Vec2;
    ///
    /// let open_linestring = LineString::new(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
    /// let closed_linestring = LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::Y, Vec2::ZERO]);
    ///
    /// assert!(open_linestring.to_ring().is_none());
    /// assert!(closed_linestring.to_ring().is_some());
    /// ```
    pub fn to_ring(&self) -> Option<Ring> {
        self.closed().then(|| Ring::new(self.0.clone()))
    }

    /// Turn this [`LineString`] into a [`Ring`], adding a closing line if the it was open
    ///
    /// # Example
    ///
    /// ```
    /// # use math::LineString;
    /// # use math::Ring;
    /// # use glam::Vec2;
    ///
    /// let open_linestring = LineString::new(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
    /// assert!(!open_linestring.closed());
    ///
    /// let closed_ring : Ring = open_linestring.close();
    /// assert!(closed_ring.to_linestring().closed())
    /// ```
    pub fn close(self) -> Ring {
        Ring::new(self.0)
    }

    /// Iterate over the points of this [`LineString`]
    ///
    /// Example
    ///
    /// ```
    /// # use math::LineString;
    /// # use glam::Vec2;
    ///
    /// let linestring = LineString::new(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
    ///
    /// let mut points_iter = linestring.points();
    ///
    /// assert_eq!(points_iter.next(), Some(Vec2::X));
    /// assert_eq!(points_iter.next(), Some(Vec2::Y));
    /// assert_eq!(points_iter.next(), Some(Vec2::ONE));
    /// assert_eq!(points_iter.next(), Some(Vec2::ONE * 2.0));
    /// assert_eq!(points_iter.next(), None);
    /// ```
    pub fn points(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.0.iter().copied()
    }

    /// Iterate over the lines of this [`LineString`]
    ///
    /// Example
    ///
    /// ```
    /// # use math::LineString;
    /// # use math::Line;
    /// # use glam::Vec2;
    ///
    /// let linestring = LineString::new(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
    ///
    /// let mut lines_iter = linestring.lines();
    ///
    /// assert_eq!(lines_iter.next(), Some(Line([Vec2::X, Vec2::Y])));
    /// assert_eq!(lines_iter.next(), Some(Line([Vec2::Y, Vec2::ONE])));
    /// assert_eq!(lines_iter.next(), Some(Line([Vec2::ONE, Vec2::ONE * 2.0])));
    /// assert_eq!(lines_iter.next(), None);
    /// ```
    pub fn lines(&self) -> impl Iterator<Item = Line> + '_ {
        self.0.windows(2).map(|s| Line([s[0], s[1]]))
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct MultiLineString(pub Vec<LineString>);

impl Default for MultiLineString {
    fn default() -> Self {
        Self(vec![])
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
        LineString::new(
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

impl<TS: AsRef<[geo::LineString<f32>]>> From<&TS> for MultiLineString {
    fn from(value: &TS) -> Self {
        MultiLineString(
            value
                .as_ref()
                .iter()
                .map(|linestring| linestring.into())
                .collect(),
        )
    }
}

impl From<&MultiLineString> for Vec<geo::LineString<f32>> {
    fn from(value: &MultiLineString) -> Self {
        value
            .0
            .iter()
            .map(|linestring| linestring.into())
            .collect::<Vec<_>>()
    }
}
