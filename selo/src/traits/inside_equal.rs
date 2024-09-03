use crate::{MultiRing, Point, Polygon, Ring};

use super::IterPoints;

/// Checks whether the set of points that these primitives represents are equal.
/// Winding is not considered.
pub trait InsideEqual {
    fn inside_eq(&self, other: &Self) -> bool;
}

impl<P: Point> InsideEqual for Ring<P> {
    fn inside_eq(&self, other: &Self) -> bool {
        let len = self.points_open().len();
        if len != other.points_open().len() {
            return false;
        }
        (0..len).any(|i| {
            self.iter_points()
                .cycle()
                .take(len)
                .eq(other.iter_points().cycle().skip(i).take(len))
        }) || (0..len).any(|i| {
            self.iter_points().rev().cycle().take(len).eq(other
                .iter_points()
                .cycle()
                .skip(i)
                .take(len))
        })
    }
}

impl<P: Point> InsideEqual for MultiRing<P> {
    fn inside_eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .all(|a| other.0.iter().any(|b| a.inside_eq(b)))
    }
}

impl<P: Point> InsideEqual for Polygon<P> {
    fn inside_eq(&self, other: &Self) -> bool {
        self.exterior().inside_eq(other.exterior()) && self.interior().inside_eq(other.interior())
    }
}

#[cfg(test)]
mod inside_eq_trait {
    use super::*;
    use glam::*;

    #[test]
    fn polygon() {
        let polygon_1 = Polygon::new(
            Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::Y]),
            MultiRing::empty(),
        );
        let polygon_2 = Polygon::new(
            Ring::new(vec![Vec2::Y, Vec2::X, Vec2::ZERO]),
            MultiRing::empty(),
        );

        assert!(polygon_1.inside_eq(&polygon_1));
        assert!(polygon_1.inside_eq(&polygon_2));
    }
}
