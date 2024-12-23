use crate::{coord_to_vec2, vec2_to_coord};

use super::{Line, Ring};
use crate::point::{Point, Point2};

#[cfg(feature = "bevy_reflect")]
use bevy_reflect::{ReflectDeserialize, ReflectSerialize};

/// Represents the set of points in the lines represented by each consecutive pair of points.
///
/// There's no connecting [`Line`] between the first and the last point.
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let line = LineString(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
/// ```
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Serialize, Deserialize)
)]
pub struct LineString<P: Point>(#[serde(bound(deserialize = ""))] pub Vec<P>);

impl<P: Point> Default for LineString<P> {
    #[inline]
    fn default() -> Self {
        Self(vec![])
    }
}

impl<P: Point> LineString<P> {
    /// constructs an empty [`LineString`]
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let empty = LineString::<Vec2>::empty();
    ///
    /// assert!(empty.0.is_empty());
    /// ```
    #[inline]
    pub fn empty() -> Self {
        Self::default()
    }

    /// Creates a new [`LineString`] enforcing its invariants if necessary.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
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
    #[inline]
    pub fn new(mut points: Vec<P>) -> Self {
        points.dedup();
        LineString(points)
    }

    /// Returns whether the [`LineString`] is defining a closed shape, meaning the first and last
    /// point coincide.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let open_linestring = LineString::new(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
    /// let closed_linestring = LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::Y, Vec2::ZERO]);
    ///
    /// assert!(!open_linestring.closed());
    /// assert!(closed_linestring.closed());
    /// ```
    #[inline]
    pub fn closed(&self) -> bool {
        self.0.last() == self.0.first()
    }

    /// If this is a [`LineString`] is closed, turn it into a [`Ring`]. Otherwise returns `None`
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let open_linestring = LineString::new(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
    /// let closed_linestring = LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::Y, Vec2::ZERO]);
    ///
    /// assert!(open_linestring.to_ring().is_none());
    /// assert!(closed_linestring.to_ring().is_some());
    /// ```
    #[inline]
    pub fn to_ring(&self) -> Option<Ring<P>> {
        self.closed().then(|| Ring::new(self.0.clone()))
    }

    /// Turn this [`LineString`] into a [`Ring`], adding a closing line if the it was open
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let open_linestring = LineString::new(vec![Vec2::X, Vec2::Y, Vec2::ONE, Vec2::ONE * 2.0]);
    /// assert!(!open_linestring.closed());
    ///
    /// let closed_ring: Ring<Vec2> = open_linestring.close();
    /// assert!(closed_ring.to_linestring().closed())
    /// ```
    #[inline]
    pub fn close(self) -> Ring<P> {
        Ring::new(self.0)
    }

    /// Iterate over the lines of this [`LineString`]
    ///
    /// Example
    ///
    /// ```
    /// # use selo::prelude::*;
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
    #[inline]
    pub fn lines(&self) -> impl Iterator<Item = Line<P>> + '_ {
        self.0.windows(2).map(|s| Line([s[0], s[1]]))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[cfg_attr(
    feature = "bevy_reflect",
    derive(bevy_reflect::Reflect),
    reflect(Serialize, Deserialize)
)]
pub struct MultiLineString<P: Point>(#[serde(bound(deserialize = ""))] pub Vec<LineString<P>>);

impl<P: Point> Default for MultiLineString<P> {
    #[inline]
    fn default() -> Self {
        Self(vec![])
    }
}

// Conversions

impl<P: Point> From<&Ring<P>> for LineString<P> {
    #[inline]
    fn from(r: &Ring<P>) -> Self {
        r.to_linestring()
    }
}

impl<P: Point2> From<&geo::LineString<P::S>> for LineString<P> {
    #[inline]
    fn from(ls: &geo::LineString<P::S>) -> Self {
        LineString::new(ls.0.iter().map(|c| coord_to_vec2(*c)).collect())
    }
}

impl<P: Point2> From<&LineString<P>> for geo::LineString<P::S> {
    #[inline]
    fn from(r: &LineString<P>) -> Self {
        Self(r.0.iter().map(|c| vec2_to_coord(*c)).collect())
    }
}

impl<P: Point2, TS: AsRef<[geo::LineString<P::S>]>> From<&TS> for MultiLineString<P> {
    #[inline]
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

impl<P: Point2> From<&MultiLineString<P>> for Vec<geo::LineString<P::S>> {
    #[inline]
    fn from(value: &MultiLineString<P>) -> Self {
        value
            .0
            .iter()
            .map(|linestring| linestring.into())
            .collect::<Vec<_>>()
    }
}
