use super::{Line, MultiRing, Ring};

/// Represents the inside area of a closed [`LineString`] with an arbitrary number of holes which
/// are excluded from this area.
///
/// # Example
///
/// ```
/// # use math::Polygon;
/// # use math::{Ring, MultiRing};
/// # use glam::Vec2;
///
/// let exterior = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 * 5.0).to_vec());
/// let interior_1 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE).to_vec());
/// let interior_2 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE * 3.0).to_vec());
/// let interiors = MultiRing(vec![interior_1, interior_2]);
///
/// let polygon = Polygon::new(exterior, interiors);
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "bevy", derive(Reflect))]
pub struct Polygon(pub Ring, pub MultiRing);

impl Polygon {
    /// Creates a new [`Polygon`] from a [`Ring`] and a [`MultiRing`]
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Polygon;
    /// # use math::{Ring, MultiRing};
    /// # use glam::Vec2;
    ///
    /// let exterior = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 * 5.0).to_vec());
    /// let interior_1 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE).to_vec());
    /// let interior_2 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE * 3.0).to_vec());
    /// let interiors = MultiRing(vec![interior_1, interior_2]);
    ///
    /// let polygon = Polygon::new(exterior, interiors);
    /// ```
    pub fn new(exterior: Ring, interior: MultiRing) -> Self {
        Self(exterior, interior)
    }

    /// Access the exterior [`Ring`] that defines the base area
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Polygon;
    /// # use math::{Ring, MultiRing};
    /// # use glam::Vec2;
    ///
    /// let exterior = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 * 5.0).to_vec());
    /// let interior_1 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE).to_vec());
    /// let interior_2 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE * 3.0).to_vec());
    /// let interiors = MultiRing(vec![interior_1, interior_2]);
    ///
    /// let polygon = Polygon::new(exterior.clone(), interiors);
    ///
    /// assert_eq!(polygon.exterior(), &exterior);
    /// ```
    pub fn exterior(&self) -> &Ring {
        &self.0
    }

    /// Access the interior [`Ring`]s that define the area of the holes that will be subtracted
    /// from the base area
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Polygon;
    /// # use math::{Ring, MultiRing};
    /// # use glam::Vec2;
    ///
    /// let exterior = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 * 5.0).to_vec());
    /// let interior_1 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE).to_vec());
    /// let interior_2 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE * 3.0).to_vec());
    /// let interiors = MultiRing(vec![interior_1, interior_2]);
    ///
    /// let polygon = Polygon::new(exterior, interiors.clone());
    ///
    /// assert_eq!(polygon.interior(), &interiors);
    /// ```
    pub fn interior(&self) -> &MultiRing {
        &self.1
    }

    /// Iterate over all the [`Line`]s of the [`Polygon`]. This includes both the lines of the
    /// [`exterior`] and the [`interior`].
    ///
    /// The order of iteration is:
    ///
    /// - exterior lines first
    /// - interior lines in the order they were specified on construction
    ///
    /// [`exterior`]: `Polygon::exterior`
    /// [`interior`]: `Polygon::interior`
    ///
    /// # Example
    ///
    /// ```
    /// # use math::Polygon;
    /// # use math::{Ring, MultiRing};
    /// # use math::Line;
    /// # use glam::Vec2;
    ///
    /// let exterior = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 * 5.0).to_vec());
    /// let interior_1 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE).to_vec());
    /// let interior_2 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE * 3.0).to_vec());
    /// let interiors = MultiRing(vec![interior_1, interior_2]);
    ///
    /// let polygon = Polygon::new(exterior, interiors);
    ///
    /// let mut iter = polygon.lines();
    ///
    /// assert_eq!(iter.next(), Some(Line([Vec2::ZERO, Vec2::X * 5.0])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::X * 5.0, Vec2::ONE * 5.0])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::ONE * 5.0, Vec2::Y * 5.0])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::Y * 5.0, Vec2::ZERO * 5.0])));
    ///
    /// assert_eq!(iter.next(), Some(Line([Vec2::ONE, Vec2::X + Vec2::ONE])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::X + Vec2::ONE, Vec2::ONE * 2.0])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::ONE * 2.0, Vec2::Y + Vec2::ONE])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::Y + Vec2::ONE, Vec2::ONE])));
    ///
    /// assert_eq!(iter.next(), Some(Line([Vec2::ONE * 3.0, Vec2::X + Vec2::ONE * 3.0])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::X + Vec2::ONE * 3.0, Vec2::ONE * 4.0])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::ONE * 4.0, Vec2::Y + Vec2::ONE * 3.0])));
    /// assert_eq!(iter.next(), Some(Line([Vec2::Y + Vec2::ONE * 3.0, Vec2::ONE * 3.0])));
    ///
    /// assert_eq!(iter.next(), None);
    /// ```
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

impl MultiPolygon {
    /// constructs an empty [`MultiPolygon`]
    ///
    /// # Example
    ///
    /// ```
    /// # use math::MultiPolygon;
    ///
    /// let empty = MultiPolygon::empty();
    ///
    /// assert!(empty.0.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self::default()
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