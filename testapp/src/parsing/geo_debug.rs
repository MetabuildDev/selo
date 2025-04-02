//! Since `0.7.16`, `geo` uses a `wkt`-like `Debug` output that isn't quite compatible with `wkt`.
//! This parses the extra object types not supported by `wkt`.
//! Example:
//! ```text
//! Polygon { exterior: LineString([Coord { x: 173.45856, y: 77.282646 }, Coord { x: 154.34856, y: 119.78603 }, Coord { x: 143.0181, y: 114.67684 }, Coord { x: 161.94347, y: 72.56411 }, Coord { x: 162.9144, y: 70.43421 }, Coord { x: 174.25348, y: 75.52239 }, Coord { x: 173.45856, y: 77.282646 }]), interiors: [] }
//! ```

use bevy::math::Vec2;
use selo::{Geometry, Triangle};
use winnow::{
    ascii::{float, multispace0},
    combinator::{alt, delimited},
    ModalResult, Parser,
};

use super::rust_debug::debug_array;

pub fn parse<'s>(input: &mut &'s str) -> ModalResult<Vec<Geometry<Vec2>>> {
    alt((
        debug_array(0.., parse_debug_single),
        parse_debug_single.map(|g| vec![g]),
    ))
    .parse_next(input)
}

pub fn parse_debug_single<'s>(input: &mut &'s str) -> ModalResult<Geometry<Vec2>> {
    parse_debug_triangle
        .map(|g| Geometry::Triangle(g))
        .parse_next(input)
}

fn parse_debug_triangle<'s>(input: &mut &'s str) -> ModalResult<Triangle<Vec2>> {
    delimited(
        "TRIANGLE(",
        winnow::combinator::seq!(
            parse_debug_coord,
            _: ",",
            parse_debug_coord,
            _: ",",
            parse_debug_coord,
        ),
        ")",
    )
    .map(|p| Triangle([p.0, p.1, p.2]))
    .parse_next(input)
}

fn parse_debug_coord<'s>(input: &mut &'s str) -> ModalResult<Vec2> {
    winnow::combinator::seq!(
        float,
        _: multispace0,
        float
    )
    .map(|p| Vec2::new(p.0, p.1))
    .parse_next(input)
}
