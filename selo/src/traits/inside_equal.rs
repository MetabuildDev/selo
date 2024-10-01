use crate::{MultiPolygon, MultiRing, Point, Polygon, Ring};

use super::IterPoints;

/// Checks whether the set of points that these primitives represents are equal.
/// Opposite windings are considered different.
pub trait InsideEqual {
    type Epsilon;

    fn inside_eq(&self, other: &Self) -> bool;

    fn inside_abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool;
}

impl<P: Point> InsideEqual for Ring<P> {
    type Epsilon = P::S;

    #[inline]
    fn inside_eq(&self, other: &Self) -> bool {
        let len = self.points_open().len();
        if len != other.points_open().len() {
            return false;
        }
        let first = self.points_open().first();
        (0..len)
            .filter(|&i| other.points_open().get(i) == first)
            .any(|i| {
                self.iter_points().cycle().take(len).eq(other
                    .iter_points()
                    .cycle()
                    .skip(i)
                    .take(len))
            })
    }

    fn inside_abs_diff_eq(&self, other: &Self, epsilon: P::S) -> bool {
        let len = self.points_open().len();
        if len != other.points_open().len() {
            return false;
        }
        let first = self.points_open().first();
        (0..len)
            .filter(|&i| other.points_open().get(i) == first)
            .any(|i| {
                self.iter_points()
                    .cycle()
                    .take(len)
                    .zip(other.iter_points().cycle().skip(i).take(len))
                    .all(|(a, b)| a.abs_diff_eq(b, epsilon))
            })
    }
}

impl<P: Point> InsideEqual for MultiRing<P> {
    type Epsilon = P::S;

    #[inline]
    fn inside_eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .all(|a| other.0.iter().any(|b| a.inside_eq(b)))
    }

    fn inside_abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .all(|a| other.0.iter().any(|b| a.inside_abs_diff_eq(b, epsilon)))
    }
}

impl<P: Point> InsideEqual for Polygon<P> {
    type Epsilon = P::S;

    #[inline]
    fn inside_eq(&self, other: &Self) -> bool {
        self.exterior().inside_eq(other.exterior()) && self.interior().inside_eq(other.interior())
    }
    #[inline]
    fn inside_abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        self.exterior()
            .inside_abs_diff_eq(other.exterior(), epsilon)
            && self
                .interior()
                .inside_abs_diff_eq(other.interior(), epsilon)
    }
}

impl<P: Point> InsideEqual for MultiPolygon<P> {
    type Epsilon = P::S;

    #[inline]
    fn inside_eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .all(|a| other.0.iter().any(|b| a.inside_eq(b)))
    }
    #[inline]
    fn inside_abs_diff_eq(&self, other: &Self, epsilon: Self::Epsilon) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .all(|a| other.0.iter().any(|b| a.inside_abs_diff_eq(b, epsilon)))
    }
}

#[cfg(test)]
mod inside_eq_trait {
    use super::*;
    use bevy_math::*;

    #[test]
    fn polygon() {
        let polygon_1 = Polygon::new(
            Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::Y]),
            MultiRing::empty(),
        );
        let polygon_2 = Polygon::new(
            Ring::new(vec![Vec2::X, Vec2::Y, Vec2::ZERO]),
            MultiRing::empty(),
        );

        assert!(polygon_1.inside_eq(&polygon_1));
        assert!(polygon_1.inside_eq(&polygon_2));
    }
}
