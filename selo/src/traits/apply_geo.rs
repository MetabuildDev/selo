use super::{ToGeo, ToSelo};

pub trait ApplyGeo<To: ToSelo>
where
    for<'a> &'a Self: ToGeo,
{
    fn apply_geo(&self, f: impl Fn(<&Self as ToGeo>::GeoType) -> To) -> To::SeloType;
}

impl<To: ToSelo, T> ApplyGeo<To> for T
where
    for<'a> &'a T: ToGeo,
{
    fn apply_geo(&self, f: impl Fn(<&Self as ToGeo>::GeoType) -> To) -> To::SeloType {
        let geo = self.to_geo();
        f(geo).to_selo()
    }
}
