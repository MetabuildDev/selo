use num_traits::Float;

use crate::utils::{coord_to_vec2, vec2_to_coord};

use crate::point::{Point, Point2};
use crate::{ToGeo, ToSelo};

/// A 2D Line
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let line = Line([Vec2::X, Vec2::Y]);
/// ```
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Line<P: Point>(pub [P; 2]);

impl<P: Point> Line<P> {
    pub fn src(&self) -> P {
        self.0[0]
    }
    pub fn dst(&self) -> P {
        self.0[1]
    }

    pub fn center(&self) -> P {
        (self.0[0] + self.0[1]) / <<P as Point>::S as From<f32>>::from(2f32)
    }

    pub fn to_dst(&self) -> P {
        self.0[1] - self.0[0]
    }

    pub fn length(&self) -> P::S {
        let to_dst = self.to_dst();
        to_dst.dot(to_dst).sqrt()
    }

    pub fn swap_coords(&self) -> Self {
        Self([self.dst(), self.src()])
    }

    pub fn pos_scaled(&self, t: P::S) -> P {
        self.0[0] + (self.0[1] - self.0[0]) * t
    }
    pub fn scalar_of(&self, p: P) -> P::S {
        let v = p - self.src();
        let to_dst = self.to_dst();
        to_dst.dot(v) / to_dst.dot(to_dst)
    }
}

// Conversions

impl<P: Point2> From<geo::Line<P::S>> for Line<P> {
    fn from(ls: geo::Line<P::S>) -> Self {
        Line([coord_to_vec2(ls.start), coord_to_vec2(ls.end)])
    }
}

impl<P: Point2> From<Line<P>> for geo::Line<P::S> {
    fn from(line: Line<P>) -> Self {
        geo::Line::new(vec2_to_coord(line.src()), vec2_to_coord(line.dst()))
    }
}

impl<P: Point2> ToGeo for Line<P> {
    type GeoType = geo::Line<P::S>;
}

impl<P: Point2> ToSelo<Line<P>> for geo::Line<P::S> {}
