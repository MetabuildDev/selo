use anyhow::{anyhow, bail, Context, Result};
use itertools::Itertools;
use selo::prelude::*;
use wkt::Wkt;

use super::Geometry;

pub fn parse_2d(s: &str) -> Result<Vec<Geometry<Vec2>>> {
    let wkt: Wkt<f32> = s.parse().map_err(|s| anyhow!("{s}"))?;
    to_geometry(wkt)
}

pub fn parse_3d(s: &str) -> Result<Vec<Geometry<Vec3>>> {
    let wkt: Wkt<f32> = s.parse().map_err(|s| anyhow!("{s}"))?;
    to_geometry(wkt)
}

fn to_geometry<P: FromWktPoint>(wkt: wkt::Wkt<f32>) -> Result<Vec<Geometry<P>>> {
    Ok(match wkt {
        Wkt::LineString(line_string) => vec![Geometry::LineString(to_linestring(&line_string)?)],
        Wkt::Polygon(polygon) => vec![Geometry::Polygon(Polygon(
            to_ring(&polygon.0[0])?,
            MultiRing(
                polygon.0[1..]
                    .iter()
                    .map(|ls| to_ring(ls))
                    .collect::<Result<_>>()?,
            ),
        ))],
        Wkt::MultiLineString(multi_line_string) => {
            vec![Geometry::MultiLineString(MultiLineString(
                multi_line_string
                    .0
                    .iter()
                    .map(|v| to_linestring(v))
                    .collect::<Result<_>>()?,
            ))]
        }
        Wkt::MultiPolygon(multi_polygon) => vec![Geometry::MultiPolygon(MultiPolygon(
            multi_polygon
                .0
                .iter()
                .map(|poly| to_polygon(poly))
                .collect::<Result<_>>()?,
        ))],
        Wkt::GeometryCollection(geometry) => geometry
            .0
            .into_iter()
            .map(to_geometry)
            .flatten_ok()
            .collect::<Result<Vec<_>>>()?,
        _ => bail!("unsupported geometry type"),
    })
}

fn to_polygon<P: FromWktPoint>(polygon: &wkt::types::Polygon<f32>) -> Result<Polygon<P>> {
    Ok(Polygon(
        to_ring(&polygon.0[0])?,
        MultiRing(
            polygon.0[1..]
                .iter()
                .map(|ls| to_ring::<P>(ls))
                .collect::<Result<_>>()?,
        ),
    ))
}

fn to_linestring<P: FromWktPoint>(ls: &wkt::types::LineString<f32>) -> Result<LineString<P>> {
    Ok(LineString(
        ls.0.iter()
            .map(|p| P::from_wkt_coord(p.clone()))
            .collect::<Result<_>>()?,
    ))
}

fn to_ring<P: FromWktPoint>(ls: &wkt::types::LineString<f32>) -> Result<Ring<P>> {
    if ls.0.first() != ls.0.last() {
        bail!("invalid Ring")
    }
    Ok(Ring::new(
        ls.0.iter()
            .map(|p| P::from_wkt_coord(p.clone()))
            .collect::<Result<Vec<_>>>()?,
    ))
}

trait FromWktPoint: selo::Point {
    fn from_wkt_coord(c: wkt::types::Coord<f32>) -> Result<Self>;
}
impl FromWktPoint for Vec2 {
    fn from_wkt_coord(c: wkt::types::Coord<f32>) -> Result<Self> {
        if c.z.is_some() {
            bail!("unexpected z coordinate")
        }
        Ok(Vec2::new(c.x, c.y))
    }
}
impl FromWktPoint for Vec3 {
    fn from_wkt_coord(c: wkt::types::Coord<f32>) -> Result<Self> {
        Ok(Vec3::new(c.x, c.y, c.z.context("missing z coordinate")?))
    }
}
