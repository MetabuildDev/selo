use crate::{MultiPolygon, Point};

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
