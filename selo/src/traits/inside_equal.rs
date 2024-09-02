use crate::{MultiRing, Point, Polygon, Ring};

use super::IterPoints;

/// Checks whether the set of points that these primitives represents are equal.
/// Winding is not considered.
pub trait InsideEqual {
    fn inside_eq(&self, other: &Self) -> bool;
}

impl<P: Point> InsideEqual for Ring<P> {
    fn inside_eq(&self, other: &Self) -> bool {
        if self.points_open().len() != other.points_open().len() {
            return false;
        }
        (0..self.points_open().len()).any(|i| {
            self.iter_points()
                .cycle()
                .eq(other.iter_points().cycle().skip(i))
        }) || (0..self.points_open().len()).any(|i| {
            self.iter_points()
                .rev()
                .cycle()
                .eq(other.iter_points().cycle().skip(i))
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
