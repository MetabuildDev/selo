use crate::errors::GeometryError;
use crate::utils::{coord_to_vec2, vec2_to_coord};

use crate::point::{Point, Point2};
use crate::{SeloScalar, ToSelo};

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
    #[inline]
    pub fn new(a: P, b: P) -> Result<Self, GeometryError> {
        if a == b {
            return Err(GeometryError::InvalidGeometry);
        }
        Ok(Self([a, b]))
    }

    #[inline]
    pub fn src(&self) -> P {
        self.0[0]
    }

    #[inline]
    pub fn dst(&self) -> P {
        self.0[1]
    }

    #[inline]
    pub fn center(&self) -> P {
        (self.0[0] + self.0[1]) / <<P as Point>::S as From<f32>>::from(2f32)
    }

    #[inline]
    pub fn to_dst(&self) -> P {
        self.0[1] - self.0[0]
    }

    #[inline]
    pub fn dir(&self) -> P {
        (self.0[1] - self.0[0]).normalize()
    }

    #[inline]
    pub fn length(&self) -> P::S {
        self.to_dst().norm()
    }

    #[inline]
    pub fn pos_scaled(&self, t: P::S) -> P {
        self.0[0] + (self.0[1] - self.0[0]) * t
    }

    #[inline]
    pub fn scalar_of(&self, p: P) -> P::S {
        let v = p - self.src();
        let to_dst = self.to_dst();
        to_dst.dot(v) / to_dst.dot(to_dst)
    }

    #[inline]
    pub fn scalar_of_normed(&self, p: P) -> P::S {
        self.scalar_of(p) * self.length()
    }

    #[inline]
    pub fn project(&self, p: P) -> P {
        let d = self.to_dst();
        let v = p - self.src();
        d * (v.dot(d) / d.dot(d)) + self.src()
    }
}

// Conversions

impl<P: Point2> From<geo::Line<P::S>> for Line<P> {
    #[inline]
    fn from(ls: geo::Line<P::S>) -> Self {
        Line([coord_to_vec2(ls.start), coord_to_vec2(ls.end)])
    }
}

impl<P: Point2> From<Line<P>> for geo::Line<P::S> {
    #[inline]
    fn from(line: Line<P>) -> Self {
        geo::Line::new(vec2_to_coord(line.src()), vec2_to_coord(line.dst()))
    }
}

impl<S: SeloScalar> ToSelo for geo::Line<S> {
    type SeloType = Line<S::Point2>;
    #[inline]
    fn to_selo(self) -> Self::SeloType {
        self.into()
    }
}
