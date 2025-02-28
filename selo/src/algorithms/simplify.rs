use crate::{IterPoints, MultiPolygon, MultiRing, Point, Polygon, Ring};

pub trait SimplifyGrouped<P: Point> {
    fn simplify(&self, eps: P::S) -> Option<Self>
    where
        Self: Sized;
}

impl<P: Point> SimplifyGrouped<P> for Ring<P> {
    fn simplify(&self, eps: P::S) -> Option<Self> {
        let mut groups = self
            .iter_points()
            .fold(Vec::<Vec<P>>::default(), |mut groups, elem| {
                if let Some(last_group) = groups.last_mut().filter(|last_group| {
                    // it's enough to check the last point since the points in the polygon are
                    // ordered and the groups are connected through consecutive overlaps
                    let last_point = last_group.last().cloned().expect("at least one point");
                    let distance: P = elem.sub(last_point);
                    distance.norm_squared() < eps * eps
                }) {
                    last_group.push(elem);
                } else {
                    groups.push(vec![elem]);
                }
                groups
            });

        // check if the first and the last group need to be merged
        let points = self.points_open();
        if points
            .first()
            .cloned()
            .and_then(|a| points.last().cloned().map(|b| (a - b)))
            .is_some_and(|diff: P| diff.norm_squared() < eps * eps)
        {
            let last_group = groups.pop();
            if let Some(first_group) = groups.first_mut() {
                first_group.extend(last_group.into_iter().flatten());
            }
        }

        (groups.len() >= 3)
            .then(|| {
                groups
                    .into_iter()
                    .map(|group| {
                        let len = group.len() as f32;
                        group.into_iter().sum::<P>() / P::S::from(len)
                    })
                    .collect::<Vec<_>>()
            })
            .map(Ring::new)
    }
}

impl<P: Point> SimplifyGrouped<P> for MultiRing<P> {
    fn simplify(&self, eps: <P as Point>::S) -> Option<Self>
    where
        Self: Sized,
    {
        self.iter()
            .map(|r| r.simplify(eps))
            .collect::<Option<_>>()
            .map(Self)
    }
}

impl<P: Point> SimplifyGrouped<P> for Polygon<P> {
    fn simplify(&self, eps: <P as Point>::S) -> Option<Self>
    where
        Self: Sized,
    {
        let exterior = self.exterior().simplify(eps)?;
        let interior = self.interior().simplify(eps).unwrap_or_default();
        Some(Polygon::new(exterior, interior))
    }
}

impl<P: Point> SimplifyGrouped<P> for MultiPolygon<P> {
    fn simplify(&self, eps: <P as Point>::S) -> Option<Self>
    where
        Self: Sized,
    {
        self.iter()
            .map(|r| r.simplify(eps))
            .collect::<Option<_>>()
            .map(Self)
    }
}

#[cfg(test)]
mod simplify_tests {
    use crate::prelude::*;

    #[test]
    fn two_points_simplified() {
        let eps = 0.01;
        let r = Ring::new(vec![
            Vec2::ZERO,
            Vec2::X,
            Vec2::ONE,
            Vec2::Y + eps * 0.1,
            Vec2::Y - eps * 0.1,
        ]);
        let expected = Some(Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]));

        assert_eq!(r.simplify(eps), expected);
    }

    #[test]
    fn four_points_simplified() {
        let eps = 0.01;
        let r = Ring::new(vec![
            Vec2::ZERO,
            Vec2::X,
            Vec2::ONE,
            Vec2::Y + eps * 0.1 * Vec2::X,
            Vec2::Y + eps * 0.1 * Vec2::Y,
            Vec2::Y - eps * 0.1 * Vec2::X,
            Vec2::Y - eps * 0.1 * Vec2::Y,
        ]);
        let expected = Some(Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]));

        assert_eq!(r.simplify(eps), expected);
    }

    #[test]
    fn simplification_start_end() {
        let eps = 0.01;
        let r = Ring::new(vec![
            Vec2::ZERO + eps * 0.1 * Vec2::Y,
            Vec2::X,
            Vec2::ONE,
            Vec2::Y,
            Vec2::ZERO - eps * 0.1 * Vec2::Y,
        ]);
        let expected = Some(Ring::new(vec![Vec2::ZERO, Vec2::X, Vec2::ONE, Vec2::Y]));

        assert_eq!(r.simplify(eps), expected);
    }

    #[test]
    fn too_few_points() {
        let eps = 0.01;
        let r = Ring::new(vec![
            Vec2::ZERO + eps * 0.1 * Vec2::Y,
            Vec2::X,
            Vec2::ZERO - eps * 0.1 * Vec2::Y,
        ]);
        let expected = None;

        assert_eq!(r.simplify(eps), expected);
    }
}
