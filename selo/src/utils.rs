use geo::*;

use crate::Point2;

#[inline]
pub(crate) fn cast_coord<From: GeoFloat, To: GeoFloat>(c: Coord<From>) -> Coord<To> {
    Coord {
        x: To::from(c.x).unwrap(),
        y: To::from(c.y).unwrap(),
    }
}

#[inline]
pub(crate) fn coord_to_vec2<P: Point2>(coord: geo::Coord<P::S>) -> P {
    P::new(coord.x, coord.y)
}

#[inline]
pub(crate) fn vec2_to_coord<P: Point2>(vec2: P) -> geo::Coord<P::S> {
    geo::Coord {
        x: vec2.x(),
        y: vec2.y(),
    }
}
