use crate::{
    Line, LineString, MultiLineString, MultiPolygon, MultiRing, Point, Polygon, Ring, Triangle,
};

use super::IterPoints;

pub trait Map<PIn, POut> {
    type Output;
    fn map(&self, f: impl Fn(PIn) -> POut) -> Self::Output;
}

impl<PIn, POut, T: Map<PIn, POut>> Map<PIn, POut> for Vec<T> {
    type Output = Vec<T::Output>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> Self::Output {
        self.iter().map(|v| v.map(&f)).collect()
    }
}

impl<PIn: Point, POut: Point> Map<PIn, POut> for Line<PIn> {
    type Output = Line<POut>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> Line<POut> {
        Line(self.0.map(f))
    }
}

impl<PIn: Point, POut: Point> Map<PIn, POut> for Triangle<PIn> {
    type Output = Triangle<POut>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> Triangle<POut> {
        Triangle(self.0.map(f))
    }
}

impl<PIn: Point, POut: Point> Map<PIn, POut> for LineString<PIn> {
    type Output = LineString<POut>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> LineString<POut> {
        LineString(self.0.iter().copied().map(&f).collect())
    }
}

impl<PIn: Point, POut: Point> Map<PIn, POut> for MultiLineString<PIn> {
    type Output = MultiLineString<POut>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> MultiLineString<POut> {
        MultiLineString(self.0.iter().map(|ls| ls.map(&f)).collect())
    }
}

impl<PIn: Point, POut: Point> Map<PIn, POut> for Ring<PIn> {
    type Output = Ring<POut>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> Ring<POut> {
        Ring::new(self.iter_points().map(&f).collect::<Vec<_>>())
    }
}

impl<PIn: Point, POut: Point> Map<PIn, POut> for MultiRing<PIn> {
    type Output = MultiRing<POut>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> MultiRing<POut> {
        MultiRing(self.0.iter().map(|r| r.map(&f)).collect())
    }
}

impl<PIn: Point, POut: Point> Map<PIn, POut> for Polygon<PIn> {
    type Output = Polygon<POut>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> Polygon<POut> {
        Polygon(self.0.map(&f), self.1.map(f))
    }
}

impl<PIn: Point, POut: Point> Map<PIn, POut> for MultiPolygon<PIn> {
    type Output = MultiPolygon<POut>;

    fn map(&self, f: impl Fn(PIn) -> POut) -> MultiPolygon<POut> {
        MultiPolygon(self.0.iter().map(|p| p.map(&f)).collect())
    }
}
