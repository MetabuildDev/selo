use crate::{primitives::*, Point};

use super::IterPoints;

/// Flip winding of a primitive
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
