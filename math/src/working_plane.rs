use bevy_math::*;
use bevy_reflect::Reflect;
use primitives::InfinitePlane3d;

#[derive(Debug, Clone, Copy, PartialEq, Reflect)]
pub struct WorkingPlane {
    pub plane: InfinitePlane3d,
    pub origin: Vec3,
}

impl WorkingPlane {
    pub fn from_normal_and_origin(normal: Dir3, origin: Vec3) -> Self {
        Self {
            plane: InfinitePlane3d::new(normal),
            origin,
        }
    }

    pub fn from_three_points([a, b, c]: [Vec3; 3]) -> Self {
        let (plane, origin) = InfinitePlane3d::from_points(a, b, c);
        Self { plane, origin }
    }

    pub fn from_points(points: impl IntoIterator<Item = Vec3>) -> Self {
        let points = points.into_iter().collect::<Vec<_>>();

        if points.len() < 3 {
            panic!("Need at least three non-colinear points to construct a plane");
        }

        let count = points.len();
        let normal = (0..count)
            .map(|idx| {
                let prev = (idx - 1 + count) % count;
                let nxt = (idx + 1) % count;
                [prev, idx, nxt]
            })
            .filter_map(|[a, b, c]| {
                Some([points.get(a)?, points.get(b)?, points.get(c)?].map(|c| *c))
            })
            .map(|[a, b, c]| (b - a).cross(c - a).normalize_or_zero())
            .sum::<Vec3>()
            .normalize();

        let center = points.into_iter().sum::<Vec3>() / count as f32;

        Self {
            plane: InfinitePlane3d::new(normal),
            origin: center,
        }
    }

    /// puts the origin at the position with minimum distance to Vec3::ZERO
    pub fn normalize(self) -> Self {
        let projection_scalar = self.origin.dot(self.plane.normal.as_vec3());

        let new_origin = if projection_scalar != 0.0 {
            self.plane.normal.as_vec3() * projection_scalar
        } else {
            // in this case the plane already is running through the origin
            Vec3::ZERO
        };

        Self {
            origin: new_origin,
            ..self
        }
    }
}
