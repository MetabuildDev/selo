use crate::{Normed, Point, Wedge};

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

/// Generalized normal:
///
/// In 2d, this is either 1.0 or -1.0 depending on the winding
/// In 3d, this is the normal of the polygon
pub trait Normal {
    type P: Point;

    fn normal(&self) -> <Self::P as Wedge>::Output;
}

/// TODO: Specialize this implementation for better performance
///
/// This trait is equivalent of normalizing `Area::area`, but implementing it
/// separately should allow us to use a faster implementation
impl<T: Area> Normal for T {
    type P = T::P;

    fn normal(&self) -> <Self::P as Wedge>::Output {
        self.area().normalize()
    }
}

#[cfg(test)]
mod tests {

    use glam::Vec2;

    use crate::{Area, Ring};

    #[test]
    fn small_area_not_zero() {
        let ring = Ring::new([
            Vec2::new(-695.88074, 517.5617),
            Vec2::new(-695.88074, 517.38007),
            Vec2::new(-695.97156, 517.4709),
        ]);

        assert_eq!(ring.area(), -0.008248329);
    }
}
