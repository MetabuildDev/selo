use std::{
    fmt::Debug,
    iter::Sum,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

use geo::{CoordNum, GeoFloat};
use num_traits::Float;

// Vector space equipped with a dot & wedge products
pub trait Point:
    Debug
    + Clone
    + Copy
    + PartialEq
    + Add<Output = Self>
    + AddAssign
    + Sub<Output = Self>
    + SubAssign
    + Sum
    + Mul<Self::S, Output = Self>
    + Div<Self::S, Output = Self>
    + Wedge<Scalar = Self::S>
    + Dot<Output = Self::S>
    + Send
    + Sync
    + 'static
{
    type S: SeloScalar;

    fn abs_diff_eq(self, rhs: Self, max_abs_diff: Self::S) -> bool {
        let diff = self.sub(rhs);
        diff.dot(diff) < max_abs_diff * max_abs_diff
    }
}

// Dot product
pub trait Dot {
    type Output: Float;
    fn dot(self, rhs: Self) -> Self::Output;
}

impl Dot for glam::Vec2 {
    type Output = f32;
    fn dot(self, rhs: Self) -> Self::Output {
        glam::Vec2::dot(self, rhs)
    }
}
impl Dot for glam::DVec2 {
    type Output = f64;
    fn dot(self, rhs: Self) -> Self::Output {
        glam::DVec2::dot(self, rhs)
    }
}
impl Dot for glam::Vec3 {
    type Output = f32;
    fn dot(self, rhs: Self) -> Self::Output {
        glam::Vec3::dot(self, rhs)
    }
}
impl Dot for glam::DVec3 {
    type Output = f64;
    fn dot(self, rhs: Self) -> Self::Output {
        glam::DVec3::dot(self, rhs)
    }
}

// Wedge product (also known as exterior product): https://en.wikipedia.org/wiki/Exterior_algebra
// This generalizes the cross product to any dimension
pub trait Wedge {
    type Scalar: Float;
    type Output: Clone
        + Copy
        + Add<Output = Self::Output>
        + AddAssign
        + Sub<Output = Self::Output>
        + SubAssign
        + Sum
        + Mul<Self::Scalar, Output = Self::Output>
        + Div<Self::Scalar, Output = Self::Output>;
    fn wedge(self, rhs: Self) -> Self::Output;
}

impl Wedge for glam::Vec2 {
    type Scalar = f32;
    type Output = f32;
    fn wedge(self, rhs: Self) -> Self::Output {
        self.perp_dot(rhs)
    }
}
impl Wedge for glam::DVec2 {
    type Scalar = f64;
    type Output = f64;
    fn wedge(self, rhs: Self) -> Self::Output {
        self.perp_dot(rhs)
    }
}
impl Wedge for glam::Vec3 {
    type Scalar = f32;
    type Output = glam::Vec3;
    fn wedge(self, rhs: Self) -> Self::Output {
        self.cross(rhs)
    }
}
impl Wedge for glam::DVec3 {
    type Scalar = f64;
    type Output = glam::DVec3;
    fn wedge(self, rhs: Self) -> Self::Output {
        self.cross(rhs)
    }
}

impl Point for glam::Vec2 {
    type S = f32;
}
impl Point for glam::Vec3 {
    type S = f32;
}
impl Point for glam::DVec2 {
    type S = f64;
}
impl Point for glam::DVec3 {
    type S = f64;
}

pub trait Point2: Point<S = Self::S2> + Wedge<Output = Self::S> {
    // This is only needed to wire up the bounds.
    // Without it, there is no way to specify that `<Self::S as SeloScalar>::Point2` must be equal to Self
    type S2: SeloScalar<Point2 = Self>;

    fn x(self) -> Self::S;
    fn y(self) -> Self::S;
    fn new(x: Self::S, y: Self::S) -> Self;
}
impl Point2 for glam::Vec2 {
    type S2 = Self::S;

    fn x(self) -> Self::S {
        self.x
    }
    fn y(self) -> Self::S {
        self.y
    }
    fn new(x: Self::S, y: Self::S) -> Self {
        Self { x, y }
    }
}
impl Point2 for glam::DVec2 {
    type S2 = Self::S;

    fn x(self) -> Self::S {
        self.x
    }
    fn y(self) -> Self::S {
        self.y
    }
    fn new(x: Self::S, y: Self::S) -> Self {
        Self { x, y }
    }
}

pub trait Point3: Point<S = Self::S3> {
    // This is only needed to wire up the bounds.
    // Without it, there is no way to specify that `<Self::S as SeloScalar>::Point3` must be equal to Self
    type S3: SeloScalar<Point3 = Self>;

    fn x(self) -> Self::S;
    fn y(self) -> Self::S;
    fn z(self) -> Self::S;
    fn new(x: Self::S, y: Self::S, z: Self::S) -> Self;
}
impl Point3 for glam::Vec3 {
    type S3 = Self::S;

    fn x(self) -> Self::S3 {
        self.x
    }
    fn y(self) -> Self::S3 {
        self.y
    }
    fn z(self) -> Self::S3 {
        self.z
    }

    fn new(x: Self::S3, y: Self::S3, z: Self::S3) -> Self {
        Self { x, y, z }
    }
}
impl Point3 for glam::DVec3 {
    type S3 = Self::S;

    fn x(self) -> Self::S3 {
        self.x
    }

    fn y(self) -> Self::S3 {
        self.y
    }
    fn z(self) -> Self::S3 {
        self.z
    }

    fn new(x: Self::S3, y: Self::S3, z: Self::S3) -> Self {
        Self { x, y, z }
    }
}

pub trait SeloScalar: Float + Debug + CoordNum + GeoFloat + /* for spade boolops */ From<f32> + Into<f64> {
    type Point2: Point2<S2 = Self>;
    type Point3: Point3<S3 = Self>;
}

impl SeloScalar for f32 {
    type Point2 = glam::Vec2;
    type Point3 = glam::Vec3;
}
impl SeloScalar for f64 {
    type Point2 = glam::DVec2;
    type Point3 = glam::DVec3;
}
