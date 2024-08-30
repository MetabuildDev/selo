use crate::{Point, Wedge};

/// Generalized area
/// In 2d, this gives the signed area of the shape
/// In 3d, this gives the normal of the shape scaled by its area
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
