use crate::{Dot, MultiPolygon, MultiRing, Point, Point2, Polygon, Ring, Wedge};

use super::{Flip, Normal};

pub trait Orient {
    type P2: Point;
    fn orient(&self, direction: <Self::P2 as Wedge>::Output) -> Self;
}

pub trait Orient2d: Orient {
    fn orient_default(&self) -> Self;

    fn orient_reversed(&self) -> Self;
}

impl<P: Point> Orient for Ring<P> {
    type P2 = P;
    fn orient(&self, direction: <Self::P2 as Wedge>::Output) -> Self {
        if self.normal().dot(direction) >= P::S::from(0.0) {
            self.clone()
        } else {
            self.flip()
        }
    }
}

impl<P: Point> Orient for Polygon<P> {
    type P2 = P;
    fn orient(&self, direction: <Self::P2 as Wedge>::Output) -> Self {
        Polygon(
            self.exterior().orient(direction),
            MultiRing(
                self.interior()
                    .into_iter()
                    .map(|ring| ring.orient(-direction))
                    .collect(),
            ),
        )
    }
}

pub const DIRECTION_DEFAULT: f32 = 1.0;
pub const DIRECTION_REVERSED: f32 = -1.0;

impl<P: Point> Orient for MultiPolygon<P> {
    type P2 = P;
    fn orient(&self, direction: <Self::P2 as Wedge>::Output) -> Self {
        MultiPolygon(self.0.iter().map(|x| x.orient(direction)).collect())
    }
}

impl<T: Orient> Orient2d for T
where
    T::P2: Point2,
{
    fn orient_default(&self) -> Self {
        self.orient(<T::P2 as Point2>::S2::from(DIRECTION_DEFAULT))
    }

    fn orient_reversed(&self) -> Self {
        self.orient(<T::P2 as Point2>::S2::from(DIRECTION_REVERSED))
    }
}
