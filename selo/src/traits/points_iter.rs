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
