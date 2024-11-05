//! Parse debug logs from geo objects
//! Example:
//! ```
//! Polygon { exterior: LineString([Coord { x: 173.45856, y: 77.282646 }, Coord { x: 154.34856, y: 119.78603 }, Coord { x: 143.0181, y: 114.67684 }, Coord { x: 161.94347, y: 72.56411 }, Coord { x: 162.9144, y: 70.43421 }, Coord { x: 174.25348, y: 75.52239 }, Coord { x: 173.45856, y: 77.282646 }]), interiors: [] }
//! ```

use bevy::math::Vec2;
use selo::prelude::*;
use winnow::{
    ascii::float,
    combinator::{alt, cut_err, delimited, separated, separated_pair},
    prelude::*,
};

use super::Geometry;

pub fn parse<'s>(input: &mut &'s str) -> PResult<Vec<Geometry<Vec2>>> {
    alt((
        cut_err(delimited(
            '[',
            separated(0.., parse_debug_single, ", "),
            ']',
        )),
        cut_err(parse_debug_single.map(|g| vec![g])),
    ))
    .parse_next(input)
}

pub fn parse_debug_single<'s>(input: &mut &'s str) -> PResult<Geometry<Vec2>> {
    alt((
        parse_debug_multipolygon.map(|g| Geometry::MultiPolygon(g)),
        parse_debug_polygon.map(|g| Geometry::Polygon(g)),
        parse_debug_multiring.map(|g| Geometry::MultiRing(g)),
        parse_debug_ring.map(|g| Geometry::Ring(g)),
    ))
    .parse_next(input)
}

fn parse_debug_multipolygon<'s>(input: &mut &'s str) -> PResult<MultiPolygon<Vec2>> {
    delimited(
        "MultiPolygon([",
        cut_err(separated(0.., parse_debug_polygon, ", ")),
        "])",
    )
    .map(|polygons| MultiPolygon(polygons))
    .parse_next(input)
}

fn parse_debug_polygon<'s>(input: &mut &'s str) -> PResult<Polygon<Vec2>> {
    delimited(
        "Polygon(",
        cut_err(separated_pair(
            parse_debug_ring,
            ", ",
            parse_debug_multiring,
        )),
        ")",
    )
    .map(|(exterior, interiors)| Polygon(exterior, interiors))
    .parse_next(input)
}

fn parse_debug_multiring<'s>(input: &mut &'s str) -> PResult<MultiRing<Vec2>> {
    delimited(
        "MultiRing([",
        cut_err(separated(0.., parse_debug_ring, ", ")),
        "])",
    )
    .map(|rings| MultiRing(rings))
    .parse_next(input)
}

fn parse_debug_ring<'s>(input: &mut &'s str) -> PResult<Ring<Vec2>> {
    delimited(
        "Ring([",
        cut_err(separated(0.., parse_debug_coord, ", ")),
        "])",
    )
    .map(|points: Vec<_>| Ring::new(points))
    .parse_next(input)
}

fn parse_debug_coord<'s>(input: &mut &'s str) -> PResult<Vec2> {
    delimited(
        "Coord { x: ",
        cut_err(separated_pair(float, ", y: ", float)),
        " }",
    )
    .map(|(x, y)| Vec2::new(x, y))
    .parse_next(input)
}
