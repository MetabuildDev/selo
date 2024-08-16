use bevy_math::*;
use primitives::InfinitePlane3d;

#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "bevy_reflect", derive(bevy_reflect::Reflect))]
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

    /// Create a new `WorkingPlane` based on three points and compute the geometric center
    /// of those points.
    ///
    /// The direction of the plane normal is determined by the winding order
    /// of the triangular shape formed by the points.
    ///
    /// # Panics
    ///
    /// Panics if a valid normal can not be computed, for example when the points
    /// are *collinear* and lie on the same line.
    pub fn from_three_points([a, b, c]: [Vec3; 3]) -> Self {
        let (plane, origin) = InfinitePlane3d::from_points(a, b, c);
        Self { plane, origin }
    }

    /// puts the origin at the position with minimum distance to Vec3::ZERO
    ///
    /// In theory it would be enough to represent the plane by
    ///
    /// - normal
    /// - distance to Vec3::ZERO
    ///
    /// now and we could omit the origin point, as it can be calculated by `normal * distance`
    pub fn hesse_normal_form(self) -> Self {
        let projection_scalar = self.origin.dot(self.plane.normal.as_vec3());
        let new_origin = self.plane.normal.as_vec3() * projection_scalar;
        Self {
            origin: new_origin,
            ..self
        }
    }

    pub fn origin(&self) -> Vec3 {
        self.origin
    }

    pub fn normal(&self) -> Dir3 {
        self.plane.normal
    }

    pub fn xy_projection(&self) -> Affine3A {
        let rotation = Quat::from_rotation_arc(self.plane.normal.as_vec3(), Vec3::Z);
        let transformed_origin = rotation * self.origin;
        Affine3A::from_translation(-Vec3::Z * transformed_origin.z) * Affine3A::from_quat(rotation)
    }

    pub fn xy_injection(&self) -> Affine3A {
        self.xy_projection().inverse()
    }

    pub fn xy_projection_injection(&self) -> (Affine3A, Affine3A) {
        let projection = self.xy_projection();
        (projection, projection.inverse())
    }

    pub fn project_point(&self, pos: Vec3) -> Vec3 {
        let dist = self.plane.normal.dot(pos - self.origin);
        pos - dist * self.plane.normal
    }
}
