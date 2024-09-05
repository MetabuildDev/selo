/// Provides serialization as/deserialization from WKT.
/// These modules are meant to be used with serde's with field attribute.
/// See: https://serde.rs/field-attrs.html#with

/// 3D ring as POLYGON
pub mod ring3_polygon {
    use std::fmt::Write;
    use std::{fmt::Display, str::FromStr};

    use crate::prelude::*;
    use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

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
        P::S3: FromStr,
        <P::S3 as FromStr>::Err: Display,
    {
        let wkt = String::deserialize(d)?.trim().to_uppercase();

        if !wkt.starts_with("POLYGON Z ((") && !wkt.starts_with("LINESTRING Z (") {
            return Err(de::Error::custom("invalid wkt"))?;
        }

        let start = wkt
            .rfind('(')
            .ok_or_else(|| de::Error::custom("invalid wkt"))?;
        let stop = wkt
            .find(')')
            .ok_or_else(|| de::Error::custom("invalid wkt"))?;
        let coords: Vec<P> = wkt[(start + 1)..stop]
            .split(',')
            .map(|coord| {
                let mut axis = coord.trim().split(' ');
                Ok(P::new(
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S3>()
                        .map_err(|e| de::Error::custom(e))?,
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S3>()
                        .map_err(|e| de::Error::custom(e))?,
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S3>()
                        .map_err(|e| de::Error::custom(e))?,
                ))
            })
            .collect::<Result<_, D::Error>>()?;

        Ok(Ring::new(coords))
    }
}

/// 2D ring as POLYGON
pub mod ring2_polygon {
    use std::fmt::Write;
    use std::{fmt::Display, str::FromStr};

    use crate::prelude::*;
    use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

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
        P::S: FromStr,
        <P::S as FromStr>::Err: Display,
    {
        let wkt = String::deserialize(d)?.trim().to_uppercase();

        if !wkt.starts_with("POLYGON Z ((") && !wkt.starts_with("LINESTRING Z (") {
            return Err(de::Error::custom("invalid wkt"))?;
        }

        let start = wkt
            .rfind('(')
            .ok_or_else(|| de::Error::custom("invalid wkt"))?;
        let stop = wkt
            .find(')')
            .ok_or_else(|| de::Error::custom("invalid wkt"))?;
        let coords: Vec<P> = wkt[(start + 1)..stop]
            .split(',')
            .map(|coord| {
                let mut axis = coord.trim().split(' ');
                Ok(P::new(
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S>()
                        .map_err(|e| de::Error::custom(e))?,
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S>()
                        .map_err(|e| de::Error::custom(e))?,
                ))
            })
            .collect::<Result<_, D::Error>>()?;

        Ok(Ring::new(coords))
    }
}

/// 3D ring as LINESTRING
pub mod ring3_linestring {
    use std::fmt::Write;
    use std::{fmt::Display, str::FromStr};

    use crate::prelude::*;
    use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

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
        P::S3: FromStr,
        <P::S3 as FromStr>::Err: Display,
    {
        let wkt = String::deserialize(d)?.trim().to_uppercase();

        if !wkt.starts_with("POLYGON Z ((") && !wkt.starts_with("LINESTRING Z (") {
            return Err(de::Error::custom("invalid wkt"))?;
        }

        let start = wkt
            .rfind('(')
            .ok_or_else(|| de::Error::custom("invalid wkt"))?;
        let stop = wkt
            .find(')')
            .ok_or_else(|| de::Error::custom("invalid wkt"))?;
        let coords: Vec<P> = wkt[(start + 1)..stop]
            .split(',')
            .map(|coord| {
                let mut axis = coord.trim().split(' ');
                Ok(P::new(
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S3>()
                        .map_err(|e| de::Error::custom(e))?,
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S3>()
                        .map_err(|e| de::Error::custom(e))?,
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S3>()
                        .map_err(|e| de::Error::custom(e))?,
                ))
            })
            .collect::<Result<_, D::Error>>()?;

        Ok(Ring::new(coords))
    }
}

/// 2D ring as LINESTRING
pub mod ring2_linestring {
    use std::fmt::Write;
    use std::{fmt::Display, str::FromStr};

    use crate::prelude::*;
    use serde::{de, ser, Deserialize, Deserializer, Serialize, Serializer};

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
        P::S: FromStr,
        <P::S as FromStr>::Err: Display,
    {
        let wkt = String::deserialize(d)?.trim().to_uppercase();

        if !wkt.starts_with("POLYGON Z ((") && !wkt.starts_with("LINESTRING Z (") {
            return Err(de::Error::custom("invalid wkt"))?;
        }

        let start = wkt
            .rfind('(')
            .ok_or_else(|| de::Error::custom("invalid wkt"))?;
        let stop = wkt
            .find(')')
            .ok_or_else(|| de::Error::custom("invalid wkt"))?;
        let coords: Vec<P> = wkt[(start + 1)..stop]
            .split(',')
            .map(|coord| {
                let mut axis = coord.trim().split(' ');
                Ok(P::new(
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S>()
                        .map_err(|e| de::Error::custom(e))?,
                    axis.next()
                        .ok_or(de::Error::custom("missing axis"))?
                        .trim()
                        .parse::<P::S>()
                        .map_err(|e| de::Error::custom(e))?,
                ))
            })
            .collect::<Result<_, D::Error>>()?;

        Ok(Ring::new(coords))
    }
}
