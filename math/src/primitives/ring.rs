use std::iter::once;

use glam::Vec2;
use itertools::Itertools as _;

use super::{Line, LineString, Polygon};

/// Represents the inside area of a closed [`LineString`].
///
/// The first coordinate is different from the last, the line connecting them is implied.
///
/// # Example
///
/// ```
/// # use math::Ring;
/// # use glam::Vec2;
///
/// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
/// ```
#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Ring(Vec<Vec2>);

impl Default for Ring {
    fn default() -> Self {
        Self::new(vec![])
    }
}

impl Ring {
    /// Creates a new [`Ring`] enforcing its invariants if necessary. This means that the
    /// constructor accepts both open and closed lists of [`Vec2`](glam::Vec2).
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Ring;
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
    pub fn new(mut points: Vec<Vec2>) -> Self {
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
    /// # use math::Ring;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// assert_eq!(ring.points_open(), &[Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    /// ```
    pub fn points_open(&self) -> &[Vec2] {
        &self.0
    }

    /// Iterates over all the points of the [`Ring`] with `first != last``.
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Ring;
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
    pub fn iter_points_open(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.0.iter().copied()
    }

    /// Iterates over all the points of the [`Ring`] with `first == last`.
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Ring;
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
    pub fn iter_points_closed(&self) -> impl Iterator<Item = Vec2> + '_ {
        self.0.iter().chain(self.0.first()).copied()
    }

    /// Converts the [`Ring`] to a closed [`LineString`] with `first == last`.
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Ring;
    /// # use math::LineString;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let linestring = ring.to_linestring();
    ///
    /// assert_eq!(linestring, LineString::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y, Vec2::ZERO]));
    /// ```
    pub fn to_linestring(&self) -> LineString {
        LineString::new(self.0.iter().cloned().chain(once(self.0[0])).collect())
    }

    /// Converts the [`Ring`] to a [`Polygon`] without holes.
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Ring;
    /// # use math::Polygon;
    /// # use math::MultiRing;
    /// # use glam::Vec2;
    ///
    /// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
    ///
    /// let polygon = ring.to_polygon();
    ///
    /// assert_eq!(polygon, Polygon::new(Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]), MultiRing::empty()));
    /// ```
    pub fn to_polygon(&self) -> Polygon {
        Polygon(self.clone(), Default::default())
    }

    /// Iterates over the [`Line`]s of the [`Ring`], including the closing line.
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Ring;
    /// # use math::Line;
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
    pub fn lines(&self) -> impl Iterator<Item = Line> + '_ {
        self.0
            .iter()
            .circular_tuple_windows()
            .map(|(a, b)| Line([*a, *b]))
    }
}

#[derive(Debug, Clone, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct MultiRing(pub Vec<Ring>);

impl Default for MultiRing {
    fn default() -> Self {
        Self(vec![])
    }
}

impl MultiRing {
    /// constructs an empty [`MultiRing`]
    ///
    /// # Example
    ///
    /// ```
    /// # use math::MultiRing;
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
