//! Parse debug logs from selo objects
//! Example:
//! ```
//! Polygon(Ring([Vec2(0.0, 0.0), Vec2(5.0, 0.0), Vec2(5.0, 5.0), Vec2(0.0, 5.0)]), MultiRing([Ring([Vec2(1.0, 1.0), Vec2(2.0, 1.0), Vec2(2.0, 2.0), Vec2(1.0, 2.0)]), Ring([Vec2(3.0, 3.0), Vec2(4.0, 3.0), Vec2(4.0, 4.0), Vec2(3.0, 4.0)])]))
//! ```

use bevy::math::Vec2;
use selo::prelude::*;
use winnow::{
    ascii::{float, multispace0},
    combinator::{alt, cut_err, delimited, separated, separated_pair},
    prelude::*,
    token::take_until,
};

use super::Geometry;

pub trait ParsablePoint: Point + Sized {
    fn parse<'s>(input: &mut &'s str) -> PResult<Self>;
}

impl ParsablePoint for Vec2 {
    fn parse<'s>(input: &mut &'s str) -> PResult<Self> {
        delimited(
            ("Vec2(", multispace0),
            cut_err(separated_pair(float, (",", multispace0), float)),
            (multispace0, ")"),
        )
        .map(|(x, y)| Vec2::new(x, y))
        .parse_next(input)
    }
}

impl ParsablePoint for Vec3 {
    fn parse<'s>(input: &mut &'s str) -> PResult<Self> {
        delimited(
            ("Vec3(", multispace0),
            cut_err(separated_pair(
                separated_pair(float, ", ", float),
                (",", multispace0),
                float,
            )),
            (multispace0, ")"),
        )
        .map(|((x, y), z)| Vec3::new(x, y, z))
        .parse_next(input)
    }
}

pub fn parse<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Vec<Geometry<P>>> {
    alt((
        delimited(
            ('[', multispace0),
            cut_err(separated(0.., parse_debug_single, (",", multispace0))),
            (multispace0, ']'),
        ),
        parse_debug_single.map(|g| vec![g]),
    ))
    .parse_next(input)
}

pub fn parse_debug_single<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Geometry<P>> {
    alt((
        parse_debug_multipolygon.map(|g| Geometry::MultiPolygon(g)),
        parse_debug_polygon.map(|g| Geometry::Polygon(g)),
        parse_debug_multiring.map(|g| Geometry::MultiRing(g)),
        parse_debug_ring.map(|g| Geometry::Ring(g)),
    ))
    .parse_next(input)
}

fn parse_debug_multipolygon<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<MultiPolygon<P>> {
    delimited(
        ("MultiPolygon(", multispace0, "[", multispace0),
        cut_err(separated(0.., parse_debug_polygon, (",", multispace0))),
        (multispace0, "]", multispace0, ")"),
    )
    .map(|polygons| MultiPolygon(polygons))
    .parse_next(input)
}

fn parse_debug_polygon<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Polygon<P>> {
    delimited(
        ("Polygon(", multispace0),
        cut_err(separated_pair(
            parse_debug_ring,
            (",", multispace0),
            parse_debug_multiring,
        )),
        (multispace0, ")"),
    )
    .map(|(exterior, interiors)| Polygon(exterior, interiors))
    .parse_next(input)
}

fn parse_debug_multiring<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<MultiRing<P>> {
    delimited(
        ("MultiRing(", multispace0, "[", multispace0),
        cut_err(separated(0.., parse_debug_ring, (",", multispace0))),
        (multispace0, "]", multispace0, ")"),
    )
    .map(|rings| MultiRing(rings))
    .parse_next(input)
}

fn parse_debug_ring<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Ring<P>> {
    delimited(
        ("Ring(", multispace0, "[", multispace0),
        cut_err(separated(0.., P::parse, (",", multispace0))),
        (multispace0, "]", multispace0, ")"),
    )
    .map(|points: Vec<_>| Ring::new(points))
    .parse_next(input)
}
