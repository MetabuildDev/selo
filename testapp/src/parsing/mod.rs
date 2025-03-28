use anyhow::Result;
use bevy::prelude::info;
use selo::prelude::*;
use winnow::Parser;

mod geo_debug;
mod rust_debug;
mod selo_debug;
mod wkt;

#[derive(Debug)]
pub enum DynamicGeometries {
    Dim2(Vec<Geometry<Vec2>>),
    Dim3(Vec<Geometry<Vec3>>),
}

/// Parse any geometry
pub fn parse(mut s: &str) -> Result<DynamicGeometries> {
    Ok(match () {
        _ if (s.contains("Vec2")
            || s.contains("Vec3")
            || s.contains("DVec2")
            || s.contains("DVec3"))
            && !s.contains("new") =>
        {
            if s.contains("DVec2") {
                info!("detected selo debug (DVec2)");
                selo_debug::parse
                    .parse(&mut s)
                    .map(|g| DynamicGeometries::Dim2(g))
                    .map_err(|e| anyhow::format_err!("{e}"))?
            } else if s.contains("Vec2") {
                info!("detected selo debug (Vec2)");
                selo_debug::parse
                    .parse(&mut s)
                    .map(|g| DynamicGeometries::Dim2(g))
                    .map_err(|e| anyhow::format_err!("{e}"))?
            } else if s.contains("DVec3") {
                info!("detected selo debug (DVec3)");
                selo_debug::parse
                    .parse(&mut s)
                    .map(|g| DynamicGeometries::Dim3(g))
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
        _ if s.contains(" Z") => {
            // 3d wkt
            info!("detected 3d wkt");
            DynamicGeometries::Dim3(wkt::parse_3d(s)?)
        }
        _ => {
            // assume 2d wkt
            info!("assuming 2d wkt");
            DynamicGeometries::Dim2(wkt::parse_2d(s)?)
        }
    })
}
