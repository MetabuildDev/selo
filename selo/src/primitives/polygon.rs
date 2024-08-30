use super::{Line, MultiRing, Ring};
use crate::{
    point::{Point, Point2},
    Area, ToGeo, ToSelo, Wedge,
};

/// Represents the inside area of a closed [`LineString`] with an arbitrary number of holes which
/// are excluded from this area.
///
/// # Example
///
/// ```
/// # use selo::prelude::*;
///
/// let exterior = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 * 5.0).to_vec());
/// let interior_1 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE).to_vec());
/// let interior_2 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE * 3.0).to_vec());
/// let interiors = MultiRing(vec![interior_1, interior_2]);
///
/// let polygon = Polygon::new(exterior, interiors);
/// ```
#[derive(Debug, Clone, Default, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct Polygon<P: Point>(pub Ring<P>, pub MultiRing<P>);

impl<P: Point> Polygon<P> {
    /// Creates a new [`Polygon`] from a [`Ring`] and a [`MultiRing`]
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let exterior = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 * 5.0).to_vec());
    /// let interior_1 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE).to_vec());
    /// let interior_2 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|vec2| vec2 + Vec2::ONE * 3.0).to_vec());
    /// let interiors = MultiRing(vec![interior_1, interior_2]);
    ///
    /// let polygon = Polygon::new(exterior, interiors);
    /// ```
    pub fn new(exterior: Ring<P>, interior: MultiRing<P>) -> Self {
        Self(exterior, interior)
    }

    /// Access the exterior [`Ring`] that defines the base area
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
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
    pub fn exterior(&self) -> &Ring<P> {
        &self.0
    }

    /// Access the interior [`Ring`]s that define the area of the holes that will be subtracted
    /// from the base area
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
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
    pub fn interior(&self) -> &MultiRing<P> {
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
    /// # use selo::prelude::*;
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
    pub fn lines(&self) -> impl Iterator<Item = Line<P>> + '_ {
        self.0
            .lines()
            .chain(self.1 .0.iter().flat_map(|ring| ring.lines()))
    }

    pub fn to_multi(&self) -> MultiPolygon<P> {
        MultiPolygon(vec![self.clone()])
    }
}

#[derive(Debug, Clone)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
pub struct MultiPolygon<P: Point>(pub Vec<Polygon<P>>);

impl<P: Point> std::ops::Deref for MultiPolygon<P> {
    type Target = Vec<Polygon<P>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<P: Point> Default for MultiPolygon<P> {
    fn default() -> Self {
        Self(vec![])
    }
}

impl<P: Point> MultiPolygon<P> {
    /// constructs an empty [`MultiPolygon`]
    ///
    /// # Example
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let empty = MultiPolygon::<Vec2>::empty();
    ///
    /// assert!(empty.0.is_empty());
    /// ```
    pub fn empty() -> Self {
        Self::default()
    }
}

// Traits

impl<P: Point> Area for Polygon<P> {
    type P = P;
    fn area(&self) -> <P as Wedge>::Output {
        self.exterior().area() - self.interior().area()
    }
}

impl<P: Point> Area for MultiPolygon<P> {
    type P = P;
    fn area(&self) -> <P as Wedge>::Output {
        self.0.iter().map(Area::area).sum()
    }
}

// Conversions

impl<P: Point2> From<&Polygon<P>> for geo::Polygon<P::S> {
    fn from(value: &Polygon<P>) -> Self {
        geo::Polygon::new(
            value.exterior().into(),
            value.interior().0.iter().map(|r| r.into()).collect(),
        )
    }
}

impl<P: Point2> From<&geo::Polygon<P::S>> for Polygon<P> {
    fn from(value: &geo::Polygon<P::S>) -> Self {
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

impl<P: Point2> From<&geo::MultiPolygon<P::S>> for MultiPolygon<P> {
    fn from(value: &geo::MultiPolygon<P::S>) -> Self {
        MultiPolygon(value.iter().map(|poly| poly.into()).collect())
    }
}

impl<P: Point2> From<&MultiPolygon<P>> for geo::MultiPolygon<P::S> {
    fn from(value: &MultiPolygon<P>) -> Self {
        geo::MultiPolygon::new(value.0.iter().map(|poly| poly.into()).collect())
    }
}

impl<'a, P: Point2> ToGeo for &'a Polygon<P> {
    type GeoType = geo::Polygon<P::S>;
}

impl<'a, P: Point2> ToGeo for &'a MultiPolygon<P> {
    type GeoType = geo::MultiPolygon<P::S>;
}

impl<'a, P: Point2> ToSelo<Polygon<P>> for &'a geo::Polygon<P::S> {}
impl<'a, P: Point2> ToSelo<MultiPolygon<P>> for &'a geo::MultiPolygon<P::S> {}
