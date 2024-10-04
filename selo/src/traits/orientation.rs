use crate::primitives::*;
use crate::Normal;
use crate::Point;
use bevy_math::{DVec2, DVec3, Vec2, Vec3};

/// TODO: This could probably be improved by making use of the `robust` crate and thinking more
/// about an API. Currently this here suffices
///
/// Provides an approximate assessment of whether two geometries share the same orientation.
///
/// # Example
///
/// ```
/// use selo::prelude::*;
///
/// let ring1 = Ring::new(vec![
///     Vec2::new(-1.0, -1.0),
///     Vec2::new(1.0, -1.0),
///     Vec2::new(1.0, 1.0),
///     Vec2::new(-1.0, 1.0),
/// ]);
///
/// let ring2 = Ring::new(vec![
///     Vec2::new(-2.0, -2.0),
///     Vec2::new(2.0, -2.0),
///     Vec2::new(2.0, 2.0),
///     Vec2::new(-2.0, 2.0),
/// ]);
///
/// let ring3 = Ring::new(vec![
///     Vec2::new(-3.0, -3.0),
///     Vec2::new(-3.0, 3.0),
///     Vec2::new(3.0, 3.0),
///     Vec2::new(3.0, -3.0),
/// ]);
/// assert!(ring1.orient(&ring2));
/// assert!(!ring1.orient(&ring3));
/// ```
pub trait Orientation: Normal {
    type P: Point;

    fn orient_tolerance(
        &self,
        other: &Self,
        tolerance: <<Self as Orientation>::P as Point>::S,
    ) -> bool;
    fn orient(&self, other: &Self) -> bool;
}

impl Orientation for Ring<Vec2> {
    type P = Vec2;

    fn orient_tolerance(
        &self,
        other: &Self,
        _tolerance: <<Self as Orientation>::P as Point>::S,
    ) -> bool {
        self.orient(other)
    }

    fn orient(&self, other: &Self) -> bool {
        self.normal() == other.normal()
    }
}

impl Orientation for Ring<bevy_math::DVec2> {
    type P = DVec2;

    fn orient_tolerance(
        &self,
        other: &Self,
        _tolerance: <<Self as Orientation>::P as Point>::S,
    ) -> bool {
        self.orient(other)
    }

    fn orient(&self, other: &Self) -> bool {
        self.normal() == other.normal()
    }
}

impl Orientation for Ring<bevy_math::Vec3> {
    type P = Vec3;

    fn orient_tolerance(
        &self,
        other: &Self,
        tolerance: <<Self as Orientation>::P as Point>::S,
    ) -> bool {
        self.normal().abs_diff_eq(other.normal(), tolerance)
    }

    fn orient(&self, other: &Self) -> bool {
        self.orient_tolerance(other, 0.0)
    }
}

impl Orientation for Ring<bevy_math::DVec3> {
    type P = DVec3;

    fn orient_tolerance(
        &self,
        other: &Self,
        tolerance: <<Self as Orientation>::P as Point>::S,
    ) -> bool {
        self.normal().abs_diff_eq(other.normal(), tolerance)
    }

    fn orient(&self, other: &Self) -> bool {
        self.orient_tolerance(other, 0.0)
    }
}
