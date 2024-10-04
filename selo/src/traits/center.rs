use crate::primitives::*;
use crate::IterPoints;
use crate::Point;

/// Generalized center of geometry
///
/// This returns the geometric center of all points which is the basic average.
///
/// Example
///
/// ```
/// use selo::prelude::*;
///
/// let polygon = Ring::new(vec![
///     Vec2::new(0.0, 0.0),
///     Vec2::new(3.0, 0.0),
///     Vec2::new(0.0, 3.0),
/// ]);
/// assert_eq!(polygon.center(), Vec2::new(1.0, 1.0))
/// ```
pub trait Center {
    type P: Point;

    fn center(&self) -> <Self as Center>::P;
}

impl<P: Point> Center for Ring<P> {
    type P = P;
    fn center(&self) -> <Self as Center>::P {
        self.iter_points().sum::<P>() / P::S::from(self.points_open().len() as f32)
    }
}

impl<P: Point> Center for MultiRing<P> {
    type P = P;
    fn center(&self) -> <Self as Center>::P {
        self.iter().map(|ring| ring.center()).sum::<P>() / P::S::from(self.0.len() as f32)
    }
}

impl<P: Point> Center for Polygon<P> {
    type P = P;
    fn center(&self) -> <Self as Center>::P {
        self.exterior().center()
    }
}

impl<P: Point> Center for MultiPolygon<P> {
    type P = P;
    fn center(&self) -> <Self as Center>::P {
        self.iter().map(|polygon| polygon.center()).sum::<P>() / P::S::from(self.0.len() as f32)
    }
}
