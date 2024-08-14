use super::{Line, MultiRing, Ring};

#[derive(Debug, Clone, Default)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Polygon(pub Ring, pub MultiRing);

impl Polygon {
    pub fn new(exterior: Ring, interior: MultiRing) -> Self {
        Self(exterior, interior)
    }

    pub fn exterior(&self) -> &Ring {
        &self.0
    }
    pub fn interior(&self) -> &MultiRing {
        &self.1
    }

    pub fn lines(&self) -> impl Iterator<Item = Line> + '_ {
        self.0
            .lines()
            .chain(self.1 .0.iter().flat_map(|ring| ring.lines()))
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct MultiPolygon(pub Vec<Polygon>);

impl Default for MultiPolygon {
    fn default() -> Self {
        Self(vec![])
    }
}

// Conversions

impl From<&Polygon> for geo::Polygon<f32> {
    fn from(value: &Polygon) -> Self {
        geo::Polygon::new(
            value.exterior().into(),
            value.interior().0.iter().map(|r| r.into()).collect(),
        )
    }
}

impl From<&geo::Polygon<f32>> for Polygon {
    fn from(value: &geo::Polygon<f32>) -> Self {
        Polygon(
            Ring::try_from(value.exterior()).unwrap(),
            MultiRing(
                value
                    .interiors()
                    .iter()
                    .map(|r| Ring::try_from(r).unwrap())
                    .collect(),
            ),
        )
    }
}

impl From<&geo::MultiPolygon<f32>> for MultiPolygon {
    fn from(value: &geo::MultiPolygon<f32>) -> Self {
        MultiPolygon(value.iter().map(|poly| poly.into()).collect())
    }
}

impl From<&MultiPolygon> for geo::MultiPolygon<f32> {
    fn from(value: &MultiPolygon) -> Self {
        geo::MultiPolygon::new(value.0.iter().map(|poly| poly.into()).collect())
    }
}
