use crate::primitives::*;
use crate::{Point2, ToGeo};
use geo::Contains;

/// Check if this kind of geometry is containing any other geometry completely
///
/// # Example
///
/// ```
/// use selo::prelude::*;
///
/// let inner = Ring::new(vec![
///     Vec2::new(-1.0, -1.0),
///     Vec2::new(1.0, -1.0),
///     Vec2::new(1.0, 1.0),
///     Vec2::new(-1.0, 1.0),
/// ]);
///
/// let outer = Ring::new(vec![
///     Vec2::new(-2.0, -2.0),
///     Vec2::new(2.0, -2.0),
///     Vec2::new(2.0, 2.0),
///     Vec2::new(-2.0, 2.0),
/// ]);
/// assert!(outer.is_containing(&inner))
/// ```
pub trait ContainsGeometry<Other> {
    type Rhs;
    fn is_containing(&self, rhs: &Other) -> bool;
}

// this is rather repetitive because some of the types are special and don't work well with
// implementing this generically. Tbh I maybe just didn't try hard enough, feel free to give it a
// shot yourself
macro_rules! impl_contains_geom {
    ($typename:ty) => {
        impl<P: Point2> ContainsGeometry<Triangle<P>> for $typename {
            type Rhs = Triangle<P>;
            fn is_containing(&self, rhs: &Triangle<P>) -> bool {
                self.to_geo().contains(&rhs.to_geo())
            }
        }

        impl<P: Point2> ContainsGeometry<Ring<P>> for $typename {
            type Rhs = Ring<P>;
            fn is_containing(&self, rhs: &Ring<P>) -> bool {
                self.to_geo().contains(&rhs.to_geo())
            }
        }

        impl<P: Point2> ContainsGeometry<MultiRing<P>> for $typename {
            type Rhs = MultiRing<P>;
            fn is_containing(&self, rhs: &MultiRing<P>) -> bool {
                rhs.iter().all(|ring| self.is_containing(ring))
            }
        }

        impl<P: Point2> ContainsGeometry<Polygon<P>> for $typename {
            type Rhs = Polygon<P>;
            fn is_containing(&self, rhs: &Polygon<P>) -> bool {
                self.is_containing(rhs.exterior()) && self.is_containing(rhs.interior())
            }
        }

        impl<P: Point2> ContainsGeometry<MultiPolygon<P>> for $typename {
            type Rhs = MultiPolygon<P>;
            fn is_containing(&self, rhs: &MultiPolygon<P>) -> bool {
                rhs.iter().all(|polygon| self.is_containing(polygon))
            }
        }
    };
}

impl_contains_geom!(Triangle<P>);
impl_contains_geom!(Ring<P>);
impl_contains_geom!(Polygon<P>);
impl_contains_geom!(MultiPolygon<P>);

impl<P: Point2> ContainsGeometry<Triangle<P>> for MultiRing<P> {
    type Rhs = Triangle<P>;
    fn is_containing(&self, rhs: &Triangle<P>) -> bool {
        self.iter().any(|ring| ring.is_containing(rhs))
    }
}
impl<P: Point2> ContainsGeometry<Ring<P>> for MultiRing<P> {
    type Rhs = Ring<P>;
    fn is_containing(&self, rhs: &Ring<P>) -> bool {
        self.iter().any(|ring| ring.is_containing(rhs))
    }
}
impl<P: Point2> ContainsGeometry<MultiRing<P>> for MultiRing<P> {
    type Rhs = MultiRing<P>;
    fn is_containing(&self, rhs: &MultiRing<P>) -> bool {
        rhs.iter()
            .all(|ring| self.iter().any(|outer| outer.is_containing(ring)))
    }
}
impl<P: Point2> ContainsGeometry<Polygon<P>> for MultiRing<P> {
    type Rhs = Polygon<P>;
    fn is_containing(&self, rhs: &Polygon<P>) -> bool {
        self.is_containing(rhs.exterior()) && self.is_containing(rhs.interior())
    }
}
impl<P: Point2> ContainsGeometry<MultiPolygon<P>> for MultiRing<P> {
    type Rhs = MultiPolygon<P>;
    fn is_containing(&self, rhs: &MultiPolygon<P>) -> bool {
        rhs.iter().all(|polygon| self.is_containing(polygon))
    }
}
