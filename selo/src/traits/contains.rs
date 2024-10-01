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
                self.to_geo().contains(&rhs.to_polygon().to_geo())
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
impl_contains_geom!(Polygon<P>);
impl_contains_geom!(MultiPolygon<P>);

impl<P: Point2> ContainsGeometry<Triangle<P>> for Ring<P> {
    type Rhs = Triangle<P>;
    fn is_containing(&self, rhs: &Triangle<P>) -> bool {
        self.to_polygon().to_geo().contains(&rhs.to_geo())
    }
}
impl<P: Point2> ContainsGeometry<Ring<P>> for Ring<P> {
    type Rhs = Ring<P>;
    fn is_containing(&self, rhs: &Ring<P>) -> bool {
        self.to_polygon()
            .to_geo()
            .contains(&rhs.to_polygon().to_geo())
    }
}
impl<P: Point2> ContainsGeometry<MultiRing<P>> for Ring<P> {
    type Rhs = MultiRing<P>;
    fn is_containing(&self, rhs: &MultiRing<P>) -> bool {
        rhs.iter().all(|ring| self.is_containing(ring))
    }
}
impl<P: Point2> ContainsGeometry<Polygon<P>> for Ring<P> {
    type Rhs = Polygon<P>;
    fn is_containing(&self, rhs: &Polygon<P>) -> bool {
        self.is_containing(rhs.exterior()) && self.is_containing(rhs.interior())
    }
}
impl<P: Point2> ContainsGeometry<MultiPolygon<P>> for Ring<P> {
    type Rhs = MultiPolygon<P>;
    fn is_containing(&self, rhs: &MultiPolygon<P>) -> bool {
        rhs.iter().all(|polygon| self.is_containing(polygon))
    }
}

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

#[cfg(test)]
mod contains_tests {
    use super::*;
    use bevy_math::*;

    macro_rules! list_of_outer_geometry_for {
        ($inner_type:ty) => {{
            let res: Vec<Box<dyn ContainsGeometry<$inner_type, Rhs = $inner_type>>> = {
                vec![
                    Box::new(Triangle([Vec2::ZERO, Vec2::X * 1000.0, Vec2::Y * 1000.0])),
                    Box::new(Ring::new([
                        Vec2::ZERO,
                        Vec2::X * 1000.0,
                        Vec2::ONE * 1000.0,
                        Vec2::Y * 1000.0,
                    ])),
                    Box::new(
                        Ring::new([
                            Vec2::ZERO,
                            Vec2::X * 1000.0,
                            Vec2::ONE * 1000.0,
                            Vec2::Y * 1000.0,
                        ])
                        .to_multi(),
                    ),
                    Box::new(
                        Ring::new([
                            Vec2::ZERO,
                            Vec2::X * 1000.0,
                            Vec2::ONE * 1000.0,
                            Vec2::Y * 1000.0,
                        ])
                        .to_polygon(),
                    ),
                    Box::new(
                        Ring::new([
                            Vec2::ZERO,
                            Vec2::X * 1000.0,
                            Vec2::ONE * 1000.0,
                            Vec2::Y * 1000.0,
                        ])
                        .to_polygon()
                        .to_multi(),
                    ),
                ]
            };
            res
        }};
    }

    #[test]
    fn tiny_triangle_true_inner() {
        let inner = Triangle([Vec2::ZERO, Vec2::X, Vec2::Y].map(|p| p + Vec2::ONE));
        let list = list_of_outer_geometry_for!(Triangle<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_triangle_on_edge() {
        let inner = Triangle([Vec2::ZERO, Vec2::X, Vec2::Y]);
        let list = list_of_outer_geometry_for!(Triangle<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_ring_true_inner() {
        let inner = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|p| p + Vec2::ONE));
        let list = list_of_outer_geometry_for!(Ring<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_ring_on_edge() {
        let inner = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
        let list = list_of_outer_geometry_for!(Ring<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_multiring_true_inner() {
        let inner =
            Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|p| p + Vec2::ONE)).to_multi();
        let list = list_of_outer_geometry_for!(MultiRing<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_multiring_on_edge() {
        let inner = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]).to_multi();
        let list = list_of_outer_geometry_for!(MultiRing<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_polygon_true_inner() {
        let inner = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|p| p + Vec2::ONE))
            .to_polygon();
        let list = list_of_outer_geometry_for!(Polygon<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_polygon_on_edge() {
        let inner = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]).to_polygon();
        let list = list_of_outer_geometry_for!(Polygon<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_multipolygon_true_inner() {
        let inner = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|p| p + Vec2::ONE))
            .to_polygon()
            .to_multi();
        let list = list_of_outer_geometry_for!(MultiPolygon<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }

    #[test]
    fn tiny_multipolygon_on_edge() {
        let inner = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y])
            .to_polygon()
            .to_multi();
        let list = list_of_outer_geometry_for!(MultiPolygon<Vec2>);
        list.iter().for_each(|outer| {
            assert!(outer.is_containing(&inner));
        });
    }
}
