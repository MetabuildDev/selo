use super::{ToGeo, ToSelo};

pub trait ApplyGeo<Output, To: ToSelo<Output>>
where
    for<'a> &'a Self: ToGeo,
{
    fn apply_geo(&self, f: impl Fn(<&Self as ToGeo>::GeoType) -> To) -> Output;
}

// impl<T: ToGeo, Output, To: ToSelo<Output>> ApplyGeo<Vec<Output>, To> for Vec<T> {
//     fn map(&self, f: impl Fn(Self::GeoType) -> To) -> Vec<Output> {
//         self.iter().map(|v| v.map(&f)).collect()
//     }
// }

impl<Output, To: ToSelo<Output>, T> ApplyGeo<Output, To> for T
where
    for<'a> &'a T: ToGeo,
{
    fn apply_geo(&self, f: impl Fn(<&Self as ToGeo>::GeoType) -> To) -> Output {
        let geo = self.to_geo();
        f(geo).to_selo()
    }
}
