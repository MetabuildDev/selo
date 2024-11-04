use crate::{primitives::*, Point};

/// Deduplicates consecutive points of a primitive.
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let ring = Ring::new(vec![Vec2::ZERO, Vec2::new(0.01, 0.0), Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)]);
///
/// assert_eq!(ring.dedup_approx(0.1), Ring::new(vec![Vec2::ZERO, Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0), Vec2::new(0.0, 1.0)]))
/// ```
pub trait DedupPoints {
    type S;

    /// Deduplicate consecutive points with the exact same coordinates.
    fn dedup(self) -> Self;

    /// Deduplicate consecutive points within `tolerance` of each other.
    fn dedup_approx(self, tolerance: Self::S) -> Self;
}

impl<P: Point> DedupPoints for LineString<P> {
    type S = P::S;

    fn dedup(mut self) -> Self {
        self.0.dedup();
        self
    }

    fn dedup_approx(mut self, tolerance: P::S) -> Self {
        self.0.dedup_by(|a, b| a.abs_diff_eq(*b, tolerance));
        self
    }
}

impl<P: Point> DedupPoints for Ring<P> {
    type S = P::S;

    fn dedup(self) -> Self {
        Ring::new(self.0)
    }

    fn dedup_approx(self, tolerance: P::S) -> Self {
        let mut points = self.0;
        points.dedup_by(|a, b| (*a).abs_diff_eq(*b, tolerance));
        if (*points.last().unwrap()).abs_diff_eq(*points.first().unwrap(), tolerance) {
            points.pop();
        }
        Ring(points)
    }
}

impl<P: Point> DedupPoints for MultiRing<P> {
    type S = P::S;

    fn dedup(self) -> Self {
        MultiRing(self.0.into_iter().map(|r| r.dedup()).collect())
    }

    fn dedup_approx(self, tolerance: P::S) -> Self {
        MultiRing(
            self.0
                .into_iter()
                .map(|r| r.dedup_approx(tolerance))
                .collect(),
        )
    }
}

impl<P: Point> DedupPoints for Polygon<P> {
    type S = P::S;

    fn dedup(self) -> Self {
        Polygon(self.0.dedup(), self.1.dedup())
    }

    fn dedup_approx(self, tolerance: P::S) -> Self {
        Polygon(
            self.0.dedup_approx(tolerance),
            self.1.dedup_approx(tolerance),
        )
    }
}

impl<P: Point> DedupPoints for MultiPolygon<P> {
    type S = P::S;

    fn dedup(self) -> Self {
        MultiPolygon(self.0.into_iter().map(DedupPoints::dedup).collect())
    }

    fn dedup_approx(self, tolerance: P::S) -> Self {
        MultiPolygon(
            self.0
                .into_iter()
                .map(|r| r.dedup_approx(tolerance))
                .collect(),
        )
    }
}
