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

pub fn parse<'s>(input: &mut &'s str) -> PResult<Vec<Geometry>> {
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

pub fn parse_debug_single<'s>(input: &mut &'s str) -> PResult<Geometry> {
    alt((
        parse_debug_multipolygon.map(|g| Geometry::MultiPolygon(g)),
        parse_debug_polygon.map(|g| Geometry::Polygon(g)),
        parse_debug_multiring.map(|g| Geometry::MultiRing(g)),
        parse_debug_ring.map(|g| Geometry::Ring(g)),
    ))
    .parse_next(input)
}

// pub fn parse<'a>() -> impl Parser<'a, &'a str, geo::Geometry<f64>, extra::Err<Rich<'a, char, Span>>>
// {
//     choice((
//         parse_debug_multipolygon().map(|g| g.into()),
//         parse_debug_polygon().map(|g| g.into()),
//         parse_debug_multi_linestring().map(|g| g.into()),
//         parse_debug_linestring().map(|g| g.into()),
//         parse_debug_rect().map(|g| g.into()),
//         parse_debug_coord().map(|g| geo::Point(g).into()),
//     ))
// }

// fn parse_debug_multipolygon<'a>(
// ) -> impl Parser<'a, &'a str, geo::MultiPolygon<f64>, extra::Err<Rich<'a, char, Span>>> {
//     parse_debug_polygon()
//         .separated_by(just(", "))
//         .collect()
//         .delimited_by(just("MultiPolygon(["), just("])"))
//         .map(|polys: Vec<geo::Polygon>| geo::MultiPolygon::new(polys))
//         .padded()
// }

// fn parse_debug_polygon<'a>(
// ) -> impl Parser<'a, &'a str, geo::Polygon<f64>, extra::Err<Rich<'a, char, Span>>> {
//     just("Polygon { exterior: ")
//         .ignore_then(parse_debug_linestring())
//         .then_ignore(just(", interiors: ["))
//         .then(parse_debug_linestring().separated_by(just(", ")).collect())
//         .then_ignore(just("] }"))
//         .map(|(exterior, interiors)| geo::Polygon::new(exterior, interiors))
//         .padded()
// }

// fn parse_debug_multi_linestring<'a>(
// ) -> impl Parser<'a, &'a str, geo::MultiLineString<f64>, extra::Err<Rich<'a, char, Span>>> {
//     parse_debug_linestring()
//         .separated_by(just(", "))
//         .collect()
//         .delimited_by(just("MultiLineString(["), just("])"))
//         .map(|ls: Vec<geo::LineString>| geo::MultiLineString::new(ls))
//         .padded()
// }

// fn parse_debug_linestring<'a>(
// ) -> impl Parser<'a, &'a str, geo::LineString<f64>, extra::Err<Rich<'a, char, Span>>> {
//     parse_debug_coord()
//         .separated_by(just(", "))
//         .collect()
//         .delimited_by(just("LineString(["), just("])"))
//         .map(|coords: Vec<geo::Coord>| geo::LineString::new(coords))
//         .padded()
// }

// fn parse_debug_rect<'a>(
// ) -> impl Parser<'a, &'a str, geo::Polygon<f64>, extra::Err<Rich<'a, char, Span>>> {
//     parse_debug_coord()
//         .then_ignore(just(", max:").padded())
//         .then(parse_debug_coord())
//         .delimited_by(just("Rect { min:").padded(), just("}").padded())
//         .map(|(min, max)| geo::Rect::new(min, max).into())
//         .padded()
// }

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
    delimited("Ring([", cut_err(separated(0.., parse_vec2, ", ")), "])")
        .map(|points: Vec<_>| Ring::new(points))
        .parse_next(input)
}

fn parse_vec2<'s>(input: &mut &'s str) -> PResult<Vec2> {
    delimited("Vec2(", cut_err(separated_pair(float, ", ", float)), ")")
        .map(|(x, y)| Vec2::new(x, y))
        .parse_next(input)
}
