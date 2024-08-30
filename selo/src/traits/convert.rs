pub trait ToGeo
where
    Self: Into<Self::GeoType>,
{
    type GeoType;
    fn to_geo(self) -> Self::GeoType {
        self.into()
    }
}

// this trait unfortunately has to be generic on S instead of using a type parameter,
// otherwise we wouldn't be able to blanket-implement it for `P: Point2` due to P being unconstrained
pub trait ToSelo<S>: Sized + Into<S> {
    fn to_selo(self) -> S {
        self.into()
    }
}
