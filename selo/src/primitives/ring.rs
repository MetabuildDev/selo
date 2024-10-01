use itertools::Itertools as _;

use crate::coord_to_vec2;

use super::{Line, LineString, Polygon, Triangle};
use crate::point::{Point, Point2};

/// Represents the inside area of a closed [`LineString`].
///
/// The first coordinate is different from the last, the line connecting them is implied.
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Ring<P: Point>(pub(crate) Vec<P>);

impl<P: Point> Default for Ring<P> {
    #[inline]
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl<P: Point> Ring<P> {
    /// Creates a new [`Ring`] enforcing its invariants if necessary. This means that the
    /// constructor accepts both open and closed lists of [`Vec2`](bevy_math::Vec2).
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
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
    #[inline]
    pub fn new(points: impl Into<Vec<P>>) -> Self {
        let mut points = points.into();
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
    /// # use selo::prelude::*;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// assert_eq!(ring.points_open(), &[Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    /// ```
    #[inline]
    pub fn points_open(&self) -> &[P] {
        &self.0
    }

    /// Iterates over all the points of the [`Ring`] with `first == last`.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let mut iter = ring.iter_points_duplicate_endpoints();
    ///
    /// assert_eq!(iter.next(), Some(Vec2::ZERO));
    /// assert_eq!(iter.next(), Some(Vec2::X));
    /// assert_eq!(iter.next(), Some(Vec2::ONE));
    /// assert_eq!(iter.next(), Some(Vec2::Y));
    /// assert_eq!(iter.next(), Some(Vec2::ZERO));
    /// assert_eq!(iter.next(), None);
    /// ```
    #[inline]
    pub fn iter_points_duplicate_endpoints(&self) -> impl Iterator<Item = P> + '_ {
        self.0.iter().chain(self.0.first()).copied()
    }

    /// Converts the [`Ring`] to a closed [`LineString`] with `first == last`.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let linestring = ring.to_linestring();
    ///
    /// assert_eq!(linestring, LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y, Vec2::ZERO]));
    /// ```
    #[inline]
    pub fn to_linestring(&self) -> LineString<P> {
        if self.0.is_empty() {
            LineString::empty()
        } else {
            LineString::new(
                self.0
                    .iter()
                    .cloned()
                    .chain(std::iter::once(self.0[0]))
                    .collect(),
            )
        }
    }

    /// Converts the [`Ring`] to a [`Polygon`] without holes.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let polygon = ring.to_polygon();
    ///
    /// assert_eq!(polygon, Polygon::new(Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]), MultiRing::empty()));
    /// ```
    #[inline]
    pub fn to_polygon(&self) -> Polygon<P> {
        Polygon(self.clone(), Default::default())
    }

    /// Iterates over the [`Line`]s of the [`Ring`], including the closing line.
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
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
    #[inline]
    pub fn lines(&self) -> impl Iterator<Item = Line<P>> + '_ {
        self.0
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| Line([*a, *b]))
    }

    /// converts this [`Ring`] into a [`MultiRing`]. This can be useful if you need a
    /// single-ring-multiring
    #[inline]
    pub fn to_multi(self) -> MultiRing<P> {
        MultiRing(vec![self])
    }

    /// tries to set the value of the `n`th point of the [`Ring`] and returns whether the function
    /// succeeded.
    ///
    /// The attempt to set the point fails if it would result in two consecutive and identical
    /// points.
    #[inline]
    pub fn try_set_point(&mut self, i: usize, new: P) -> bool {
        let len = self.0.len();
        if self.0[(i + 1) % len] == new || self.0[(i + len - 1) % len] == new {
            return false;
        }
        self.0[i] = new;
        true
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct MultiRing<P: Point>(pub Vec<Ring<P>>);

impl<P: Point> std::ops::Deref for MultiRing<P> {
    type Target = [Ring<P>];

    fn deref(&self) -> &Self::Target {
        self.rings()
    }
}

impl<P: Point> Default for MultiRing<P> {
    #[inline]
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
    /// # use selo::prelude::*;
    ///
    /// let empty = MultiRing::<Vec2>::empty();
    ///
    /// assert!(empty.0.is_empty());
    /// ```
    #[inline]
    pub fn empty() -> Self {
        Self::default()
    }

    #[inline]
    pub fn rings(&self) -> &[Ring<P>] {
        &self.0
    }
}

// Traits

impl<P: Point> std::ops::Index<usize> for Ring<P> {
    type Output = P;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

// Conversions

impl<P: Point2> From<geo::Triangle<P::S>> for Ring<P> {
    #[inline]
    fn from(value: geo::Triangle<P::S>) -> Self {
        Self::new(value.to_array().map(|c| coord_to_vec2(c)).to_vec())
    }
}

impl<P: Point2> TryFrom<&geo::LineString<P::S>> for Ring<P> {
    type Error = ();

    #[inline]
    fn try_from(ls: &geo::LineString<P::S>) -> Result<Self, Self::Error> {
        LineString::from(ls).to_ring().ok_or(())
    }
}

impl<P: Point2> From<&Ring<P>> for geo::LineString<P::S> {
    #[inline]
    fn from(value: &Ring<P>) -> Self {
        (&value.to_linestring()).into()
    }
}
impl<P: Point2> From<Ring<P>> for geo::LineString<P::S> {
    #[inline]
    fn from(value: Ring<P>) -> Self {
        geo::LineString::from(&value)
    }
}

impl<P: Point> From<Triangle<P>> for Ring<P> {
    #[inline]
    fn from(value: Triangle<P>) -> Self {
        Ring::new(value.0.to_vec())
    }
}
