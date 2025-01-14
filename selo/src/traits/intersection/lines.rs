use crate::{algorithms::Line2DIntersection, LinesIter, Point2};

/// Check if the lines of this kind of geometry are intersecting with any of the lines of the other geometry
///
/// # Example
///
/// ```
/// use selo::prelude::*;
///
/// let ring1 = Ring::new(vec![
///     Vec2::new(-1.0, -1.0),
///     Vec2::new(1.0, -1.0),
///     Vec2::new(1.0, 1.0),
///     Vec2::new(-1.0, 1.0),
/// ]);
///
/// let ring2 = Ring::new(vec![
///     Vec2::new(0.0, 0.0),
///     Vec2::new(2.0, 0.0),
///     Vec2::new(2.0, 2.0),
///     Vec2::new(0.0, 2.0),
/// ]);
///
/// let intersection_points = ring1.intersection_points(&ring2, 0.001).collect::<Vec<_>>();
///
/// assert!(intersection_points.contains(&Vec2::new(1.0, 0.0)));
/// assert!(intersection_points.contains(&Vec2::new(0.0, 1.0)));
/// ```
pub trait LineIntersectable<P: Point2, Other>: LinesIter {
    type Rhs;
    fn line_intersections(
        &self,
        other: &Other,
        tolerance: P::S,
    ) -> impl Iterator<Item = Line2DIntersection<P>>;

    fn first_line_intersection(
        &self,
        other: &Other,
        tolerance: P::S,
    ) -> Option<Line2DIntersection<P>>;

    fn first_intersection_point(&self, other: &Other, tolerance: P::S) -> Option<P>;

    fn intersection_points(&self, other: &Other, tolerance: P::S) -> impl Iterator<Item = P> {
        self.line_intersections(other, tolerance)
            .filter_map(|intersection| {
                intersection
                    .is_true_intersection()
                    .then(|| intersection.pos())
                    .flatten()
            })
    }
}

impl<P: Point2, SelfT, OtherT> LineIntersectable<P, OtherT> for SelfT
where
    SelfT: LinesIter<P = P>,
    OtherT: LinesIter<P = P>,
{
    type Rhs = OtherT;
    fn line_intersections(
        &self,
        other: &OtherT,
        tolerance: P::S,
    ) -> impl Iterator<Item = Line2DIntersection<P>> {
        self.iter_lines()
            .flat_map(move |self_line| {
                other
                    .iter_lines()
                    .map(move |other_line| self_line.intersection(&other_line, tolerance))
            })
            .filter(|intersection| intersection.intersect())
    }

    fn first_line_intersection(
        &self,
        other: &OtherT,
        tolerance: <P>::S,
    ) -> Option<Line2DIntersection<P>> {
        self.iter_lines().find_map(move |self_line| {
            other.iter_lines().find_map(move |other_line| {
                let intersection = self_line.intersection(&other_line, tolerance);
                intersection.intersect().then_some(intersection)
            })
        })
    }

    fn first_intersection_point(&self, other: &OtherT, tolerance: <P>::S) -> Option<P> {
        self.iter_lines().find_map(move |self_line| {
            other.iter_lines().find_map(move |other_line| {
                let intersection = self_line.intersection(&other_line, tolerance);
                intersection
                    .is_true_intersection()
                    .then(|| intersection.pos())
                    .flatten()
            })
        })
    }
}

#[cfg(test)]
mod line_intersection_tests {
    use super::*;
    use crate::primitives::*;
    use bevy_math::*;

    #[test]
    fn test_name() {
        let ring1 = Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]);
        let ring2 =
            Ring::new([Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y].map(|p| p + Vec2::ONE * 0.5));

        let intersections = ring1.intersection_points(&ring2, 0.001).collect::<Vec<_>>();

        assert!(intersections.contains(&Vec2::new(1.0, 0.5)));
        assert!(intersections.contains(&Vec2::new(0.5, 1.0)));
    }
}
