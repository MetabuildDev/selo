//! Parse debug logs from geo objects
//! Example:
//! ```text
//! Polygon { exterior: LineString([Coord { x: 173.45856, y: 77.282646 }, Coord { x: 154.34856, y: 119.78603 }, Coord { x: 143.0181, y: 114.67684 }, Coord { x: 161.94347, y: 72.56411 }, Coord { x: 162.9144, y: 70.43421 }, Coord { x: 174.25348, y: 75.52239 }, Coord { x: 173.45856, y: 77.282646 }]), interiors: [] }
//! ```

use bevy::math::Vec2;
use selo::prelude::*;
use winnow::{
    ascii::{float, multispace0},
    combinator::{alt, cut_err, delimited, opt, preceded, separated, separated_pair},
    prelude::*,
};

use super::{
    rust_debug::{debug_array, debug_list},
    Geometry,
};

pub fn parse<'s>(input: &mut &'s str) -> PResult<Vec<Geometry<Vec2>>> {
    alt((
        debug_array(0.., parse_debug_single),
        parse_debug_single.map(|g| vec![g]),
    ))
    .parse_next(input)
}

pub fn parse_debug_single<'s>(input: &mut &'s str) -> PResult<Geometry<Vec2>> {
    alt((
        parse_debug_multipolygon.map(|g| Geometry::MultiPolygon(g)),
        parse_debug_polygon.map(|g| Geometry::Polygon(g)),
        parse_debug_multiring.map(|g| Geometry::MultiRing(g)),
        parse_debug_ring.map(|g| Geometry::Ring(g)),
        parse_debug_triangle.map(|g| Geometry::Triangle(g)),
        parse_debug_line.map(|g| Geometry::Line(g)),
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
        ("Polygon {", multispace0),
        cut_err(separated_pair(
            preceded(("exterior:", multispace0), parse_debug_ring),
            (",", multispace0),
            preceded(
                ("interiors:", multispace0),
                debug_array(0.., parse_debug_ring),
            ),
        )),
        (multispace0, "}"),
    )
    .map(|(exterior, interiors)| Polygon(exterior, MultiRing(interiors)))
    .parse_next(input)
}

fn parse_debug_multiring<'s>(input: &mut &'s str) -> PResult<MultiRing<Vec2>> {
    delimited(
        ("MultiLineString(", multispace0),
        debug_array(0.., parse_debug_ring),
        (multispace0, ")"),
    )
    .map(|rings| MultiRing(rings))
    .parse_next(input)
}

fn parse_debug_ring<'s>(input: &mut &'s str) -> PResult<Ring<Vec2>> {
    delimited(
        ("LineString(", multispace0),
        debug_array(0.., parse_debug_coord),
        (multispace0, ")"),
    )
    .map(|points: Vec<_>| Ring::new(points))
    .parse_next(input)
}

fn parse_debug_triangle<'s>(input: &mut &'s str) -> PResult<Triangle<Vec2>> {
    delimited(
        ("Triangle(", multispace0),
        cut_err(debug_list(3, parse_debug_coord)),
        (multispace0, ")"),
    )
    .map(|p: Vec<_>| Triangle([p[0], p[1], p[2]]))
    .parse_next(input)
}

fn parse_debug_line<'s>(input: &mut &'s str) -> PResult<Line<Vec2>> {
    delimited(
        "Line {",
        cut_err(winnow::combinator::seq!(
            _: (multispace0, "start:", multispace0),
            parse_debug_coord,
            _: (multispace0, ",", multispace0),
            _: ("end:", multispace0),
            parse_debug_coord,
            _: (opt((multispace0, ",")), multispace0))),
        ("}", opt((multispace0, ","))),
    )
    .map(|(src, dst)| Line([src, dst]))
    .parse_next(input)
}

fn parse_debug_coord<'s>(input: &mut &'s str) -> PResult<Vec2> {
    delimited(
        "Coord {",
        cut_err(winnow::combinator::seq!(
            _: (multispace0, "x:", multispace0),
            float,
            _: (multispace0, ",", multispace0),
            _: ("y:", multispace0),
            float,
            _: (opt((multispace0, ",")), multispace0))),
        "}",
    )
    .map(|(x, y)| Vec2::new(x, y))
    .parse_next(input)
}
