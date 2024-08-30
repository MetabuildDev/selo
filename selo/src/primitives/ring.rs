use std::iter::once;

use itertools::Itertools as _;

use crate::coord_to_vec2;

use super::{Line, LineString, Polygon};
use crate::point::{Point, Point2};

/// Represents the inside area of a closed [`LineString`].
///
/// The first coordinate is different from the last, the line connecting them is implied.
///
/// # Example
///
/// ```
/// # use selo::Ring;
/// # use glam::Vec2;
///
/// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Ring<P: Point>(Vec<P>);

impl<P: Point> Default for Ring<P> {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl<P: Point> Ring<P> {
    /// Creates a new [`Ring`] enforcing its invariants if necessary. This means that the
    /// constructor accepts both open and closed lists of [`Vec2`](glam::Vec2).
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::Ring;
    /// # use glam::Vec2;
    ///
    /// let ring_from_open = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    /// let ring_from_closed = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y, Vec2::ZERO]);
    ///
    /// // repetitive points are messing with most of the algorithms in 2D, so the constructor
    /// // deals with this itself and fixes it internally. Inputs like these happen on accident
    /// // most of the times anyways
    /// let ring_extremely_deduped = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::X, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let expected = vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y];
    ///
    /// assert_eq!(ring_from_open.points_open(), &expected);
    /// assert_eq!(ring_from_closed.points_open(), &expected);
    /// assert_eq!(ring_extremely_deduped.points_open(), &expected);
    /// ```
    pub fn new(mut points: Vec<P>) -> Self {
        points.dedup();
        if points.last() == points.first() {
            points.pop();
        }
        Ring(points)
    }

    /// Returns a reference to the points of the [`Ring`].
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::Ring;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// assert_eq!(ring.points_open(), &[Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    /// ```
    pub fn points_open(&self) -> &[P] {
        &self.0
    }

    /// Iterates over all the points of the [`Ring`] with `first != last``.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::Ring;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let mut iter = ring.iter_points_open();
    ///
    /// assert_eq!(iter.next(), Some(Vec2::ZERO));
    /// assert_eq!(iter.next(), Some(Vec2::X));
    /// assert_eq!(iter.next(), Some(Vec2::ONE));
    /// assert_eq!(iter.next(), Some(Vec2::Y));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_points_open(&self) -> impl Iterator<Item = P> + '_ {
        self.0.iter().copied()
    }

    /// Iterates over all the points of the [`Ring`] with `first == last`.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::Ring;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let mut iter = ring.iter_points_closed();
    ///
    /// assert_eq!(iter.next(), Some(Vec2::ZERO));
    /// assert_eq!(iter.next(), Some(Vec2::X));
    /// assert_eq!(iter.next(), Some(Vec2::ONE));
    /// assert_eq!(iter.next(), Some(Vec2::Y));
    /// assert_eq!(iter.next(), Some(Vec2::ZERO));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn iter_points_closed(&self) -> impl Iterator<Item = P> + '_ {
        self.0.iter().chain(self.0.first()).copied()
    }

    /// Converts the [`Ring`] to a closed [`LineString`] with `first == last`.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::Ring;
    /// # use selo::LineString;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let linestring = ring.to_linestring();
    ///
    /// assert_eq!(linestring, LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y, Vec2::ZERO]));
    /// ```
    pub fn to_linestring(&self) -> LineString<P> {
        LineString::new(self.0.iter().cloned().chain(once(self.0[0])).collect())
    }

    /// Converts the [`Ring`] to a [`Polygon`] without holes.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::Ring;
    /// # use selo::Polygon;
    /// # use selo::MultiRing;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let polygon = ring.to_polygon();
    ///
    /// assert_eq!(polygon, Polygon::new(Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]), MultiRing::empty()));
    /// ```
    pub fn to_polygon(&self) -> Polygon<P> {
        Polygon(self.clone(), Default::default())
    }

    /// Iterates over the [`Line`]s of the [`Ring`], including the closing line.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::Ring;
    /// # use selo::Line;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let mut iter = ring.lines();
    ///
    /// assert_eq!(iter.next(), Some(Line([Vec2::ZERO, Vec2::X])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::X, Vec2::ONE])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::ONE, Vec2::Y])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::Y, Vec2::ZERO])));
    /// assert_eq!(iter.next(), None);
    /// ```
    pub fn lines(&self) -> impl Iterator<Item = Line<P>> + '_ {
        self.0
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| Line([*a, *b]))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct MultiRing<P: Point>(pub Vec<Ring<P>>);

impl<P: Point> Default for MultiRing<P> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<P: Point> MultiRing<P> {
    /// constructs an empty [`MultiRing`]
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::MultiRing;
    ///
    /// let empty = MultiRing::empty();
    ///
    /// assert!(empty.0.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self::default()
    }
}

// Conversions

impl<P: Point2> From<geo::Triangle<P::Float>> for Ring<P> {
    fn from(value: geo::Triangle<P::Float>) -> Self {
        Self::new(value.to_array().map(|c| coord_to_vec2(c)).to_vec())
    }
}

impl<P: Point2> TryFrom<&geo::LineString<P::Float>> for Ring<P> {
    type Error = ();

    fn try_from(ls: &geo::LineString<P::Float>) -> Result<Self, Self::Error> {
        LineString::from(ls).to_ring().ok_or(())
    }
}

impl<P: Point2> From<&Ring<P>> for geo::LineString<P::Float> {
    fn from(value: &Ring<P>) -> Self {
        (&value.to_linestring()).into()
    }
}
