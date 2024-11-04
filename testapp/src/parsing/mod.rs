use std::any::type_name;

use anyhow::{anyhow, Result};
use bevy::prelude::{error, info};
use selo::prelude::*;
use winnow::Parser;

mod selo_debug;

// TODO: Support 3D

pub enum Geometry {
    Line(Line<Vec2>),
    LineString(LineString<Vec2>),
    MultiLineString(MultiLineString<Vec2>),
    Triangle(Triangle<Vec2>),
    Ring(Ring<Vec2>),
    MultiRing(MultiRing<Vec2>),
    Polygon(Polygon<Vec2>),
    MultiPolygon(MultiPolygon<Vec2>),
}

/// Parse any geometry
pub fn parse(mut s: &str) -> Result<Vec<Geometry>> {
    Ok(match () {
        _ if (s.starts_with("MultiPolygon")
            || s.starts_with("Polygon")
            || s.starts_with("MultiRing")
            || s.starts_with("Ring")
            // || s.starts_with("MultiLineString")
            // || s.starts_with("LineString")
            // || s.starts_with("Triangle")
            || s.starts_with("["))
            && !s.contains("new")
            && (s.contains("Vec2") || s.contains("Vec3")) =>
        {
            info!("detected selo debug");
            selo_debug::parse
                .parse(&mut s)
                .map_err(|e| anyhow::format_err!("{e}"))?
        }
        _ => Err(anyhow!("unrecognized input"))?,
    })
}

// pub type Span = SimpleSpan<usize>;

// fn number<'a>() -> impl Parser<'a, &'a str, f64, extra::Err<Rich<'a, char, Span>>> {
//     let exp = just('e')
//         .or(just('E'))
//         .then(one_of("+-").or_not())
//         .then(text::digits(10).clone());

//     just("-")
//         .or_not()
//         .then(text::int(10))
//         .then(just(".").then(text::digits(10).or_not()).or_not())
//         .then(exp.or_not())
//         .map_slice(|s: &str| s.parse::<f64>().unwrap())
//         .padded()
// }
