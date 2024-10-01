use crate::primitives::*;
use crate::Point2;
use crate::SeloScalar;

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

impl<S: SeloScalar> ToSelo for geo::Line<S> {
    type SeloType = Line<S::Point2>;
    #[inline]
    fn to_selo(self) -> Self::SeloType {
        self.into()
    }
}

impl<'a, S: SeloScalar> ToSelo for &'a geo::LineString<S> {
    type SeloType = LineString<S::Point2>;
    fn to_selo(self) -> Self::SeloType {
        self.into()
    }
}

impl<'a, S: SeloScalar> ToSelo for &'a geo::Polygon<S> {
    type SeloType = Polygon<S::Point2>;

    #[inline]
    fn to_selo(self) -> Self::SeloType {
        self.into()
    }
}

impl<'a, S: SeloScalar> ToSelo for &'a geo::MultiPolygon<S> {
    type SeloType = MultiPolygon<S::Point2>;

    #[inline]
    fn to_selo(self) -> Self::SeloType {
        self.into()
    }
}

impl<S: SeloScalar> ToSelo for geo::Triangle<S> {
    type SeloType = Triangle<S::Point2>;

    #[inline]
    fn to_selo(self) -> Self::SeloType {
        self.into()
    }
}
