use geo::*;

use crate::Point2;

pub(crate) fn cast_coord<From: GeoFloat, To: GeoFloat>(c: Coord<From>) -> Coord<To> {
    Coord {
        x: To::from(c.x).unwrap(),
        y: To::from(c.y).unwrap(),
    }
}

pub(crate) fn coord_to_vec2<P: Point2>(coord: geo::Coord<P::Float>) -> P {
    P::new(coord.x, coord.y)
}

pub(crate) fn vec2_to_coord<P: Point2>(vec2: P) -> geo::Coord<P::Float> {
    geo::Coord {
        x: vec2.x(),
        y: vec2.y(),
    }
}

pub(crate) fn empty_multipolygon<P: Point2>() -> MultiPolygon<P::Float> {
    MultiPolygon::new(vec![])
}
