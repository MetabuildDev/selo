use std::iter::once;

use itertools::Itertools;

use crate::{Line, LineString, MultiLineString, MultiPolygon, MultiRing, Point, Polygon, Ring};

use super::IterPoints;

pub trait LinesIter {
    type P: Point;
    fn iter_lines(&self) -> impl Iterator<Item = Line<Self::P>>;
}

impl<P: Point> LinesIter for Line<P> {
    type P = P;
    fn iter_lines(&self) -> impl Iterator<Item = Line<Self::P>> {
        once(Line(self.0))
    }
}

impl<P: Point> LinesIter for LineString<P> {
    type P = P;
    fn iter_lines(&self) -> impl Iterator<Item = Line<Self::P>> {
        self.iter_points()
            .tuple_windows()
            .map(|(a, b)| Line([a, b]))
    }
}

impl<P: Point> LinesIter for MultiLineString<P> {
    type P = P;
    fn iter_lines(&self) -> impl Iterator<Item = Line<Self::P>> {
        self.0.iter().flat_map(LinesIter::iter_lines)
    }
}

impl<P: Point> LinesIter for Ring<P> {
    type P = P;
    fn iter_lines(&self) -> impl Iterator<Item = Line<Self::P>> {
        self.iter_points()
            .circular_tuple_windows()
            .map(|(a, b)| Line([a, b]))
    }
}

impl<P: Point> LinesIter for MultiRing<P> {
    type P = P;
    fn iter_lines(&self) -> impl Iterator<Item = Line<Self::P>> {
        self.0.iter().flat_map(LinesIter::iter_lines)
    }
}

impl<P: Point> LinesIter for Polygon<P> {
    type P = P;
    fn iter_lines(&self) -> impl Iterator<Item = Line<Self::P>> {
        self.exterior()
            .iter_lines()
            .chain(self.interior().iter_lines())
    }
}

impl<P: Point> LinesIter for MultiPolygon<P> {
    type P = P;
    fn iter_lines(&self) -> impl Iterator<Item = Line<Self::P>> {
        self.0.iter().flat_map(LinesIter::iter_lines)
    }
}
