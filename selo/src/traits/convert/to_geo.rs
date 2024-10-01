use crate::primitives::*;
use geo::Coord;

use crate::Point2;

pub trait ToGeo {
    type GeoType: 'static;
    fn to_geo(self) -> Self::GeoType;
}

impl<P: Point2> ToGeo for P {
    type GeoType = geo::Coord<P::S>;

    #[inline]
    fn to_geo(self) -> Self::GeoType {
        Coord {
            x: self.x(),
            y: self.y(),
        }
    }
}

impl<T: ToGeo> ToGeo for Vec<T> {
    type GeoType = Vec<T::GeoType>;

    #[inline]
    fn to_geo(self) -> Self::GeoType {
        self.into_iter().map(ToGeo::to_geo).collect()
    }
}

impl<P: Point2> ToGeo for Line<P> {
    type GeoType = geo::Line<P::S>;
    #[inline]
    fn to_geo(self) -> Self::GeoType {
        self.into()
    }
}

impl<'a, P: Point2> ToGeo for &'a LineString<P> {
    type GeoType = geo::LineString<P::S>;

    #[inline]
    fn to_geo(self) -> Self::GeoType {
        self.into()
    }
}

impl<'a, P: Point2> ToGeo for &'a Polygon<P> {
    type GeoType = geo::Polygon<P::S>;

    #[inline]
    fn to_geo(self) -> Self::GeoType {
        self.into()
    }
}

impl<'a, P: Point2> ToGeo for &'a MultiPolygon<P> {
    type GeoType = geo::MultiPolygon<P::S>;

    #[inline]
    fn to_geo(self) -> Self::GeoType {
        self.into()
    }
}

impl<'a, P: Point2> ToGeo for &'a Ring<P> {
    type GeoType = geo::LineString<P::S>;

    #[inline]
    fn to_geo(self) -> Self::GeoType {
        self.into()
    }
}

impl<P: Point2> ToGeo for Triangle<P> {
    type GeoType = geo::Triangle<P::S>;

    #[inline]
    fn to_geo(self) -> Self::GeoType {
        self.into()
    }
}
