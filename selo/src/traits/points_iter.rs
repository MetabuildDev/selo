use crate::Point;

pub trait IterPoints {
    type P: Point;
    fn iter_points(&self) -> impl Iterator<Item = Self::P>;
}
