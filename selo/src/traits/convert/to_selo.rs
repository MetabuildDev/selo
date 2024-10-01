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
