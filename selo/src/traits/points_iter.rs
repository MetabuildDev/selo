use crate::primitives::*;
use crate::Point;

/// Iterates over all the points of the primitive
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
///
/// let mut iter = ring.iter_points();
///
/// assert_eq!(iter.next(), Some(Vec2::ZERO));
/// assert_eq!(iter.next(), Some(Vec2::X));
/// assert_eq!(iter.next(), Some(Vec2::ONE));
/// assert_eq!(iter.next(), Some(Vec2::Y));
/// assert_eq!(iter.next(), None);
/// ```
pub trait IterPoints {
    type P: Point;
    fn iter_points(&self) -> impl Iterator<Item = Self::P>;
}

impl<P: Point> IterPoints for Triangle<P> {
    type P = P;
    #[inline]
    fn iter_points(&self) -> impl Iterator<Item = Self::P> + Clone {
        self.0.iter().copied()
    }
}

impl<P: Point> IterPoints for LineString<P> {
    type P = P;
    #[inline]
    fn iter_points(&self) -> impl ExactSizeIterator<Item = P> + Clone {
        self.0.iter().copied()
    }
}

impl<P: Point> IterPoints for MultiLineString<P> {
    type P = P;
    #[inline]
    fn iter_points(&self) -> impl Iterator<Item = P> + Clone {
        self.0.iter().flat_map(IterPoints::iter_points)
    }
}

impl<P: Point> IterPoints for Polygon<P> {
    type P = P;

    #[inline]
    fn iter_points(&self) -> impl Iterator<Item = P> + Clone {
        self.exterior()
            .iter_points()
            .chain(self.interior().iter_points())
    }
}

impl<P: Point> IterPoints for MultiPolygon<P> {
    type P = P;

    #[inline]
    fn iter_points(&self) -> impl Iterator<Item = P> + Clone {
        self.iter().flat_map(IterPoints::iter_points)
    }
}

impl<P: Point> IterPoints for Ring<P> {
    type P = P;

    #[inline]
    fn iter_points(
        &self,
    ) -> impl Clone + ExactSizeIterator<Item = P> + DoubleEndedIterator<Item = P> {
        self.0.iter().copied()
    }
}

impl<P: Point> IterPoints for MultiRing<P> {
    type P = P;

    #[inline]
    fn iter_points(&self) -> impl Iterator<Item = P> + Clone {
        self.0.iter().flat_map(IterPoints::iter_points)
    }
}
