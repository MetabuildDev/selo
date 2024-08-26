mod line;
use std::fmt::Debug;

use geo::{CoordNum, GeoFloat};
pub use line::*;

mod linestring;
pub use linestring::*;

mod polygon;
pub use polygon::*;

mod ring;
pub use ring::*;

mod triangle;
pub use triangle::*;

pub trait Point: Debug + Clone + Copy + PartialEq + Send + Sync + 'static {}

impl Point for glam::Vec2 {}
impl Point for glam::Vec3 {}
impl Point for glam::DVec2 {}
impl Point for glam::DVec3 {}

pub trait Point2: Point {
    type Float: CoordNum + GeoFloat + /* for spade boolops */ From<f32> + Into<f64>;

    fn x(self) -> Self::Float;
    fn y(self) -> Self::Float;
    fn new(x: Self::Float, y: Self::Float) -> Self;
}

impl Point2 for glam::Vec2 {
    type Float = f32;

    fn x(self) -> Self::Float {
        self.x
    }

    fn y(self) -> Self::Float {
        self.y
    }

    fn new(x: Self::Float, y: Self::Float) -> Self {
        Self { x, y }
    }
}
impl Point2 for glam::DVec2 {
    type Float = f64;

    fn x(self) -> Self::Float {
        self.x
    }

    fn y(self) -> Self::Float {
        self.y
    }

    fn new(x: Self::Float, y: Self::Float) -> Self {
        Self { x, y }
    }
}
