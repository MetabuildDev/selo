use crate::{primitives::*, Point};

use super::IterPoints;

/// Flips the winding of a primitive.
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let ring = Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
/// let rev_ring = ring.flip();
///
/// let mut iter = rev_ring.lines();
///
/// assert_eq!(iter.next(), Some(Line([Vec2::Y, Vec2::ONE])));
/// assert_eq!(iter.next(), Some(Line([Vec2::ONE, Vec2::X])));
/// assert_eq!(iter.next(), Some(Line([Vec2::X, Vec2::ZERO])));
/// assert_eq!(iter.next(), Some(Line([Vec2::ZERO, Vec2::Y])));
/// assert_eq!(iter.next(), None);
/// ```
pub trait Flip {
    fn flip(&self) -> Self;
}

impl<P: Point> Flip for Line<P> {
    fn flip(&self) -> Self {
        Line([self.dst(), self.src()])
    }
}

impl<P: Point> Flip for LineString<P> {
    fn flip(&self) -> Self {
        LineString(self.0.iter().copied().rev().collect())
    }
}
impl<P: Point> Flip for MultiLineString<P> {
    fn flip(&self) -> Self {
        MultiLineString(self.0.iter().map(|r| r.flip()).collect())
    }
}

impl<P: Point> Flip for Ring<P> {
    #[inline]
    fn flip(&self) -> Self {
        Ring::new(self.iter_points().rev().collect::<Vec<_>>())
    }
}
impl<P: Point> Flip for MultiRing<P> {
    #[inline]
    fn flip(&self) -> Self {
        MultiRing(self.iter().map(|r| r.flip()).collect())
    }
}
impl<P: Point> Flip for Polygon<P> {
    #[inline]
    fn flip(&self) -> Self {
        Polygon(self.exterior().flip(), self.interior().flip())
    }
}
impl<P: Point> Flip for MultiPolygon<P> {
    #[inline]
    fn flip(&self) -> Self {
        MultiPolygon(self.iter().map(|p| p.flip()).collect())
    }
}
