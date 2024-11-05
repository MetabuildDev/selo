use anyhow::{anyhow, Result};
use bevy::prelude::info;
use selo::prelude::*;
use winnow::Parser;

mod geo_debug;
mod rust_debug;
mod selo_debug;

pub enum DynamicGeometries {
    Dim2(Vec<Geometry<Vec2>>),
    Dim3(Vec<Geometry<Vec3>>),
}

pub enum Geometry<P: Point> {
    Line(Line<P>),
    LineString(LineString<P>),
    MultiLineString(MultiLineString<P>),
    Triangle(Triangle<P>),
    Ring(Ring<P>),
    MultiRing(MultiRing<P>),
    Polygon(Polygon<P>),
    MultiPolygon(MultiPolygon<P>),
}

/// Parse any geometry
pub fn parse(mut s: &str) -> Result<DynamicGeometries> {
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
            if s.contains("Vec2") {
                info!("detected selo debug (Vec2)");
                selo_debug::parse
                    .parse(&mut s)
                    .map(|g| DynamicGeometries::Dim2(g))
                    .map_err(|e| anyhow::format_err!("{e}"))?
            } else {
                info!("detected selo debug (Vec3)");
                selo_debug::parse
                    .parse(&mut s)
                    .map(|g| DynamicGeometries::Dim3(g))
                    .map_err(|e| anyhow::format_err!("{e}"))?
            }
        }
        _ if (s.contains("Coord")) && !s.contains("new") => {
            info!("detected geo debug");
            geo_debug::parse
                .parse(&mut s)
                .map(|g| DynamicGeometries::Dim2(g))
                .map_err(|e| anyhow::format_err!("{e}"))?
        }
        _ => Err(anyhow!("unrecognized input"))?,
    })
}
