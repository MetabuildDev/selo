use crate::{Point, Wedge};

/// Generalized area
/// In 2d, this gives the signed area of the shape
/// In 3d, this gives the normal of the shape scaled by its area
///
/// Example
///
/// ```
/// use selo::prelude::*;
///
/// let polygon = Ring::new(vec![
///     Vec2::new(1.0, 1.0),
///     Vec2::new(-2.0, 4.0),
///     Vec2::new(-2.0, -2.0),
/// ]);
/// assert_eq!(polygon.area(), 9.0)
/// ```
///
pub trait Area {
    type P: Point;

    fn area(&self) -> <Self::P as Wedge>::Output;
}

/// TODO: Currently unimplemented
/// This will be the equivalent of normalizing `Area::area`, but making it a
/// separate trait should allow us to use a faster implementation
///
/// Generalized normal:
///
/// In 2d, this is either 1.0 or -1.0 depending on the winding
/// In 3d, this is the normal of the polygon
pub trait Normal {
    type P: Point;

    fn normal(&self) -> <Self::P as Wedge>::Output;
}