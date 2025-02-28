//! Parse debug logs from selo objects
//! Example:
//! ```text
//! Polygon(Ring([Vec2(0.0, 0.0), Vec2(5.0, 0.0), Vec2(5.0, 5.0), Vec2(0.0, 5.0)]), MultiRing([Ring([Vec2(1.0, 1.0), Vec2(2.0, 1.0), Vec2(2.0, 2.0), Vec2(1.0, 2.0)]), Ring([Vec2(3.0, 3.0), Vec2(4.0, 3.0), Vec2(4.0, 4.0), Vec2(3.0, 4.0)])]))
//! ```

use bevy::math::Vec2;
use selo::prelude::*;
use winnow::{
    ascii::{float, multispace0},
    combinator::{alt, cut_err, delimited, opt, seq},
    prelude::*,
};

use super::{
    rust_debug::{debug_array, debug_list},
    Geometry,
};

pub trait ParsablePoint: Point + Sized {
    fn parse<'s>(input: &mut &'s str) -> PResult<Self>;
}

impl ParsablePoint for Vec2 {
    fn parse<'s>(input: &mut &'s str) -> PResult<Self> {
        delimited(
            ("Vec2(", multispace0),
            cut_err(debug_list(2, float)),
            (multispace0, ")"),
        )
        .map(|c| Vec2::new(c[0], c[1]))
        .parse_next(input)
    }
}

impl ParsablePoint for Vec3 {
    fn parse<'s>(input: &mut &'s str) -> PResult<Self> {
        delimited(
            ("Vec3(", multispace0),
            cut_err(debug_list(3, float)),
            (multispace0, ")"),
        )
        .map(|c| Vec3::new(c[0], c[1], c[2]))
        .parse_next(input)
    }
}

pub fn parse<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Vec<Geometry<P>>> {
    alt((
        debug_array(0.., parse_debug_single),
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
        parse_debug_multilinestring.map(|g| Geometry::MultiLineString(g)),
        parse_debug_linestring.map(|g| Geometry::LineString(g)),
        parse_debug_triangle.map(|g| Geometry::Triangle(g)),
        parse_debug_line.map(|g| Geometry::Line(g)),
    ))
    .parse_next(input)
}

fn parse_debug_multipolygon<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<MultiPolygon<P>> {
    delimited(
        ("MultiPolygon(", multispace0),
        debug_array(0.., parse_debug_polygon),
        (opt((multispace0, ',')), multispace0, ")"),
    )
    .map(|polygons| MultiPolygon(polygons))
    .parse_next(input)
}

fn parse_debug_polygon<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Polygon<P>> {
    delimited(
        ("Polygon(", multispace0),
        cut_err(seq!(
            parse_debug_ring,
            _: (",", multispace0),
            parse_debug_multiring,
            _: opt((",", multispace0))
        )),
        (multispace0, ")"),
    )
    .map(|(exterior, interiors)| Polygon(exterior, interiors))
    .parse_next(input)
}

fn parse_debug_multiring<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<MultiRing<P>> {
    delimited(
        ("MultiRing(", multispace0),
        debug_array(0.., parse_debug_ring),
        (opt((multispace0, ',')), multispace0, ")"),
    )
    .map(|rings| MultiRing(rings))
    .parse_next(input)
}

fn parse_debug_ring<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Ring<P>> {
    delimited(
        ("Ring(", multispace0),
        debug_array(0.., P::parse),
        (opt((multispace0, ',')), multispace0, ")"),
    )
    .map(|points: Vec<_>| Ring::new(points))
    .parse_next(input)
}

fn parse_debug_multilinestring<'s, P: ParsablePoint>(
    input: &mut &'s str,
) -> PResult<MultiLineString<P>> {
    delimited(
        ("MultiLineString(", multispace0),
        debug_array(0.., parse_debug_linestring),
        (opt((multispace0, ',')), multispace0, ")"),
    )
    .map(|points: Vec<_>| MultiLineString(points))
    .parse_next(input)
}

fn parse_debug_linestring<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<LineString<P>> {
    delimited(
        ("LineString(", multispace0),
        debug_array(0.., P::parse),
        (opt((multispace0, ',')), multispace0, ")"),
    )
    .map(|points: Vec<_>| LineString(points))
    .parse_next(input)
}

fn parse_debug_triangle<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Triangle<P>> {
    delimited(
        ("Triangle(", multispace0),
        debug_array(3, P::parse),
        (opt((multispace0, ',')), multispace0, ")"),
    )
    .map(|points: Vec<_>| Triangle([points[0], points[1], points[2]]))
    .parse_next(input)
}

fn parse_debug_line<'s, P: ParsablePoint>(input: &mut &'s str) -> PResult<Line<P>> {
    delimited(
        ("Line(", multispace0),
        debug_array(2, P::parse),
        (opt((multispace0, ',')), multispace0, ")"),
    )
    .map(|points: Vec<_>| Line([points[0], points[1]]))
    .parse_next(input)
}
