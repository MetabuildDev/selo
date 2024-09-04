use geo::Coord;

use crate::Point2;
use crate::SeloScalar;

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

pub trait ToSelo {
    type SeloType: 'static;
    fn to_selo(self) -> Self::SeloType;
}

impl<S: SeloScalar> ToSelo for geo::Coord<S> {
    type SeloType = S::Point2;

    #[inline]
    fn to_selo(self) -> Self::SeloType {
        <S::Point2>::new(self.x, self.y)
    }
}

impl<S: SeloScalar> ToSelo for geo::Point<S> {
    type SeloType = S::Point2;
    fn to_selo(self) -> Self::SeloType {
        <S::Point2>::new(self.x(), self.y())
    }
}

impl<T: ToSelo> ToSelo for Vec<T> {
    type SeloType = Vec<T::SeloType>;

    #[inline]
    fn to_selo(self) -> Self::SeloType {
        self.into_iter().map(ToSelo::to_selo).collect()
    }
}
