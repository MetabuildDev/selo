use crate::{prelude::Workplane, primitives::*, Embed, Map, Point, ToGeo, ToSelo, Unembed};
use bevy_math::{DVec2, DVec3, Vec2, Vec3};

use super::Orient2d;

/// Expand or shrink geometry in normal direction at every point
///
/// - The `distance` determines how distant each edge of the original geometry is to each edge of the result geometry. The effect of the sign will be:
///   - `+` to expand (to add padding, make bigger, to inflate)
///   - `-` to shrink (to add margins, make smaller, to deflate)
///
/// # Note
///
/// The resulting geometry will always be a [`MultiPolygon`]. This is due to the fact that
///   - expanding geometry can create new holes
///     - a horse shoe which becomes a donut
///   - shrinking geometry can split it
///     - a banana with a thin middle will split into two ends
///
/// # Example
///
/// ```
/// use selo::prelude::*;
///
/// let polygon = Ring::new(vec![
///     Vec2::new(-1.0, -1.0),
///     Vec2::new(1.0, -1.0),
///     Vec2::new(1.0, 1.0),
///     Vec2::new(-1.0, 1.0),
/// ]);
///
/// let expected = Ring::new(vec![
///     Vec2::new(-2.0, -2.0),
///     Vec2::new(2.0, -2.0),
///     Vec2::new(2.0, 2.0),
///     Vec2::new(-2.0, 2.0),
/// ]);
/// assert_eq!(polygon.buffer(1.0)[0].exterior().clone(), expected)
/// ```
///
pub trait BufferGeometry {
    type P: Point;

    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P>;
}

impl BufferGeometry for Polygon<Vec2> {
    type P = Vec2;

    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P> {
        self.map(|p| p.as_dvec2())
            .buffer(distance)
            .map(|p| p.as_vec2())
    }
}

impl BufferGeometry for Polygon<DVec2> {
    type P = DVec2;

    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P> {
        geo_buffer::buffer_polygon(&self.orient_default().to_geo(), distance).to_selo()
    }
}

impl BufferGeometry for Polygon<Vec3> {
    type P = Vec3;

    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P> {
        Workplane::from_primitive(self)
            .map_or(MultiPolygon::<<Self as BufferGeometry>::P>::empty(), |wp| {
                self.embed(wp).buffer(distance).unembed(wp)
            })
    }
}

impl BufferGeometry for Polygon<DVec3> {
    type P = DVec3;

    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P> {
        self.map(|p| p.as_vec3())
            .buffer(distance)
            .map(|p| p.as_dvec3())
    }
}

impl BufferGeometry for MultiPolygon<Vec2> {
    type P = Vec2;
    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P> {
        self.map(|p| p.as_dvec2())
            .buffer(distance)
            .map(|p| p.as_vec2())
    }
}

impl BufferGeometry for MultiPolygon<DVec2> {
    type P = DVec2;
    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P> {
        println!("{:?}", self);
        geo_buffer::buffer_multi_polygon(&self.orient_default().to_geo(), distance).to_selo()
    }
}

impl BufferGeometry for MultiPolygon<Vec3> {
    type P = Vec3;
    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P> {
        Workplane::from_primitive(self)
            .map_or(MultiPolygon::<<Self as BufferGeometry>::P>::empty(), |wp| {
                self.embed(wp).buffer(distance).unembed(wp)
            })
    }
}

impl BufferGeometry for MultiPolygon<DVec3> {
    type P = DVec3;
    fn buffer(&self, distance: f64) -> MultiPolygon<<Self as BufferGeometry>::P> {
        self.map(|p| p.as_vec3())
            .buffer(distance)
            .map(|p| p.as_dvec3())
    }
}

impl<P> BufferGeometry for Ring<P>
where
    P: Point,
    Polygon<P>: BufferGeometry<P = P>,
{
    type P = P;

    fn buffer(&self, distance: f64) -> crate::MultiPolygon<<Self as BufferGeometry>::P> {
        self.to_polygon().buffer(distance)
    }
}

impl<P> BufferGeometry for Triangle<P>
where
    P: Point,
    Polygon<P>: BufferGeometry<P = P>,
{
    type P = P;

    fn buffer(&self, distance: f64) -> crate::MultiPolygon<<Self as BufferGeometry>::P> {
        self.to_ring().buffer(distance)
    }
}

impl<P> BufferGeometry for MultiRing<P>
where
    P: Point,
    Polygon<P>: BufferGeometry<P = P>,
{
    type P = P;

    fn buffer(&self, distance: f64) -> crate::MultiPolygon<<Self as BufferGeometry>::P> {
        self.0.iter().map(|ring| ring.buffer(distance)).fold(
            crate::MultiPolygon::empty(),
            |mut acc, mp| {
                acc.0.extend(mp.0);
                acc
            },
        )
    }
}
