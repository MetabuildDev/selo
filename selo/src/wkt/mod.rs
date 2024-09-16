use crate::SeloScalar;

/// Provides serialization as/deserialization from WKT.
/// These modules are meant to be used with serde's with field attribute.
/// See: https://serde.rs/field-attrs.html#with

/// 3D ring as POLYGON
pub mod ring3_polygon {
    use std::fmt::Write;
    use std::{fmt::Display, str::FromStr};

    use crate::prelude::*;
    use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
    use wkt::Wkt;

    pub fn serialize<S, P: Point3>(ring: &Ring<P>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        P::S3: Display,
    {
        let mut r = String::new();
        r.push_str("POLYGON Z ((");
        let points = ring.points_open();
        for p in points {
            write!(r, "{} {} {},", p.x(), p.y(), p.z()).map_err(|e| ser::Error::custom(e))?;
        }
        write!(r, "{} {} {}", points[0].x(), points[0].y(), points[0].z())
            .map_err(|e| ser::Error::custom(e))?;
        r.push_str("))");
        String::serialize(&r, s)
    }

    pub fn deserialize<'de, D, P: Point3>(d: D) -> Result<Ring<P>, D::Error>
    where
        D: Deserializer<'de>,
        P::S3: FromStr + Default,
        <P::S3 as FromStr>::Err: Display,
    {
        let wkt = String::deserialize(d)?
            .parse::<Wkt<P::S>>()
            .map_err(|e| de::Error::custom(e))?;

        let Wkt::Polygon(polygon) = wkt else {
            return Err(de::Error::custom("wrong wkt type"));
        };

        let exterior = polygon
            .0
            .get(0)
            .ok_or_else(|| de::Error::custom("missing exterior ring"))?;
        Ok(Ring::new(
            super::wkt_linestring_coords_3d(&exterior.0).map_err(de::Error::custom)?,
        ))
    }
}

/// 2D ring as POLYGON
pub mod ring2_polygon {
    use std::fmt::Write;
    use std::{fmt::Display, str::FromStr};

    use crate::prelude::*;
    use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
    use wkt::Wkt;

    pub fn serialize<S, P: Point2>(ring: &Ring<P>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        P::S: Display,
    {
        let mut r = String::new();
        r.push_str("POLYGON ((");
        let points = ring.points_open();
        for p in points {
            write!(r, "{} {},", p.x(), p.y()).map_err(|e| ser::Error::custom(e))?;
        }
        write!(r, "{} {}", points[0].x(), points[0].y()).map_err(|e| ser::Error::custom(e))?;
        r.push_str("))");
        String::serialize(&r, s)
    }

    pub fn deserialize<'de, D, P: Point2>(d: D) -> Result<Ring<P>, D::Error>
    where
        D: Deserializer<'de>,
        P::S: FromStr + Default,
        <P::S as FromStr>::Err: Display,
    {
        let wkt = String::deserialize(d)?
            .parse::<Wkt<P::S>>()
            .map_err(|e| de::Error::custom(e))?;

        let Wkt::Polygon(polygon) = wkt else {
            return Err(de::Error::custom("wrong wkt type"));
        };

        let exterior = polygon
            .0
            .get(0)
            .ok_or_else(|| de::Error::custom("missing exterior ring"))?;
        Ok(Ring::new(super::wkt_linestring_coords_2d(&exterior.0)))
    }
}

/// 3D ring as LINESTRING
pub mod ring3_linestring {
    use std::fmt::Write;
    use std::{fmt::Display, str::FromStr};

    use crate::prelude::*;
    use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
    use wkt::Wkt;

    pub fn serialize<S, P: Point3>(ring: &Ring<P>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        P::S3: Display,
    {
        let mut r = String::new();
        r.push_str("LINESTRING Z (");
        let points = ring.points_open();
        for p in points {
            write!(r, "{} {} {},", p.x(), p.y(), p.z()).map_err(|e| ser::Error::custom(e))?;
        }
        write!(r, "{} {} {}", points[0].x(), points[0].y(), points[0].z())
            .map_err(|e| ser::Error::custom(e))?;
        r.push_str(")");
        String::serialize(&r, s)
    }

    pub fn deserialize<'de, D, P: Point3>(d: D) -> Result<Ring<P>, D::Error>
    where
        D: Deserializer<'de>,
        P::S3: FromStr + Default,
        <P::S3 as FromStr>::Err: Display,
    {
        let wkt = String::deserialize(d)?
            .parse::<Wkt<P::S>>()
            .map_err(|e| de::Error::custom(e))?;

        let Wkt::LineString(ls) = wkt else {
            return Err(de::Error::custom("wrong wkt type"));
        };

        Ok(Ring::new(
            super::wkt_linestring_coords_3d(&ls.0).map_err(de::Error::custom)?,
        ))
    }
}

/// 2D ring as LINESTRING
pub mod ring2_linestring {
    use std::fmt::Write;
    use std::{fmt::Display, str::FromStr};

    use crate::prelude::*;
    use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};
    use wkt::Wkt;

    pub fn serialize<S, P: Point2>(ring: &Ring<P>, s: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
        P::S: Display,
    {
        let mut r = String::new();
        r.push_str("LINESTRING (");
        let points = ring.points_open();
        for p in points {
            write!(r, "{} {},", p.x(), p.y()).map_err(|e| ser::Error::custom(e))?;
        }
        write!(r, "{} {}", points[0].x(), points[0].y()).map_err(|e| ser::Error::custom(e))?;
        r.push_str(")");
        String::serialize(&r, s)
    }

    pub fn deserialize<'de, D, P: Point2>(d: D) -> Result<Ring<P>, D::Error>
    where
        D: Deserializer<'de>,
        P::S: FromStr + Default,
        <P::S as FromStr>::Err: Display,
    {
        let wkt = String::deserialize(d)?
            .parse::<Wkt<P::S>>()
            .map_err(|e| de::Error::custom(e))?;

        let Wkt::LineString(ls) = wkt else {
            return Err(de::Error::custom("wrong wkt type"));
        };

        Ok(Ring::new(super::wkt_linestring_coords_2d(&ls.0)))
    }
}

fn wkt_linestring_coords_2d<S: SeloScalar>(ls: &[wkt::types::Coord<S>]) -> Vec<S::Point2> {
    use crate::point::Point2;
    ls.iter()
        .map(|p| S::Point2::new(p.x, p.y))
        .collect::<Vec<_>>()
}

fn wkt_linestring_coords_3d<S: SeloScalar>(
    ls: &[wkt::types::Coord<S>],
) -> Result<Vec<S::Point3>, &'static str> {
    use crate::point::Point3;
    ls.iter()
        .map(|p| Ok(S::Point3::new(p.x, p.y, p.z.ok_or("missing z coord")?)))
        .collect::<Result<Vec<_>, _>>()
}
