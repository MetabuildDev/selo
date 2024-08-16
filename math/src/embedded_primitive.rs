use crate::prelude::WorkingPlane;

/// A trait to classify 2D geometric types that can be created from points on a 3D working plane and
/// the plane itself
pub trait Flattenable {
    /// Representation of the flat geometry in 3D coordinates
    type Type3D;

    /// method to transform the geometry from a 3D plane to the XY plane into 2D coordinates
    fn embed(repr_3d: Self::Type3D, working_plane: WorkingPlane) -> Self;

    /// method to transform the geometry from the XY plane with 2D coordinates back to a 3D plane
    fn unembed(self, working_plane: WorkingPlane) -> Self::Type3D;
}

/// This type represents geometry in a 3D context which was projected to 2D coordinates to apply
/// algorithms for problems which are typically easier to solve in 2D space.
///
/// ```
/// # use math::prelude::*;
/// # use glam::Vec3;
///
/// let [a,b,c] = [Vec3::X, Vec3::Y, Vec3::Z];
/// let plane = WorkingPlane::from_three_points([a,b,c]);
///
/// let triangle_2d = EmbeddedPrimitive::<Triangle>::new([a,b,c], plane);
/// ```
#[derive(Debug, Clone)]
pub struct FlatPrimitive<P: Flattenable> {
    primitive: P,
    working_plane: WorkingPlane,
}

impl<A: Flattenable> FlatPrimitive<A> {
    /// Transforms a given 3D geometry that is flat with respect to some [`WorkingPlane`] into 2D space
    ///
    /// ```
    /// # use math::prelude::*;
    /// # use glam::Vec3;
    ///
    /// let [a,b,c] = [Vec3::X, Vec3::Y, Vec3::Z];
    /// let plane = WorkingPlane::from_three_points([a,b,c]);
    ///
    /// let triangle_2d = EmbeddedPrimitive::<Triangle>::new([a,b,c], plane);
    /// ```
    pub fn new(from: A::Type3D, working_plane: WorkingPlane) -> Self {
        Self {
            primitive: A::embed(from, working_plane),
            working_plane,
        }
    }

    /// Apply transformations to the flattened 2D geometry
    ///
    /// ```
    /// # use math::prelude::*;
    /// # use glam::Vec3;
    ///
    /// let [a,b,c] = [Vec3::X, Vec3::Y, Vec3::Z];
    /// let plane = WorkingPlane::from_three_points([a,b,c]);
    ///
    /// let triangle_2d = EmbeddedPrimitive::<Triangle>::new([a,b,c], plane);
    ///
    /// let flip_triangle = |triangle: Triangle| -> Triangle {
    ///     Triangle(triangle.0.map(|mut vec2| {
    ///         vec2.y = -vec2.y;
    ///         vec2
    ///     }))
    /// };
    ///
    /// let flipped_triangle = triangle_2d.map_geometry(flip_triangle);
    /// ```
    pub fn map_geometry<B: Flattenable>(self, f: impl Fn(A) -> B) -> FlatPrimitive<B> {
        FlatPrimitive {
            primitive: f(self.primitive),
            working_plane: self.working_plane,
        }
    }

    /// Transform the 2D geometry back into 3D space onto the [`WorkingPlane`] where it came from.
    ///
    /// ```
    /// # use math::prelude::*;
    /// # use glam::Vec3;
    ///
    /// let [a,b,c] = [Vec3::X, Vec3::Y, Vec3::Z];
    /// let plane = WorkingPlane::from_three_points([a,b,c]);
    ///
    /// let triangle_2d = EmbeddedPrimitive::<Triangle>::new([a,b,c], plane);
    ///
    /// let flip_triangle = |triangle: Triangle| -> Triangle {
    ///     Triangle(triangle.0.map(|mut vec2| {
    ///         vec2.y = -vec2.y;
    ///         vec2
    ///     }))
    /// };
    ///
    /// let flipped_triangle = triangle_2d.map_geometry(flip_triangle);
    ///
    /// let ([a,b,c], plane) = flipped_triangle.unpack();
    /// ```
    pub fn unpack(self) -> (A::Type3D, WorkingPlane) {
        (
            A::unembed(self.primitive, self.working_plane),
            self.working_plane,
        )
    }
}

mod private_impls {
    use super::Flattenable;
    use crate::primitives::*;
    use glam::*;

    impl Flattenable for Line {
        type Type3D = [Vec3; 2];
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let proj = working_plane.xy_projection();
            Line(
                repr_3d
                    .map(|vec3| proj.transform_point3(vec3))
                    .map(|vec2| vec2.truncate()),
            )
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            let inj = working_plane.xy_injection();
            self.0
                .map(|vec2| vec2.extend(0.0))
                .map(|vec3| inj.transform_point3(vec3))
        }
    }
    impl Flattenable for Triangle {
        type Type3D = [Vec3; 3];
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let proj = working_plane.xy_projection();
            Triangle(
                repr_3d
                    .map(|vec3| proj.transform_point3(vec3))
                    .map(|vec2| vec2.truncate()),
            )
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            let inj = working_plane.xy_injection();
            self.0
                .map(|vec2| vec2.extend(0.0))
                .map(|vec3| inj.transform_point3(vec3))
        }
    }
    impl Flattenable for MultiTriangle {
        type Type3D = Vec<[Vec3; 3]>;
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            MultiTriangle(
                repr_3d
                    .into_iter()
                    .map(|triangle| Triangle::embed(triangle, working_plane))
                    .collect::<Vec<_>>(),
            )
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            self.0
                .into_iter()
                .map(|triangle| triangle.unembed(working_plane))
                .collect::<Vec<_>>()
        }
    }
    impl Flattenable for LineString {
        type Type3D = Vec<Vec3>;
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let proj = working_plane.xy_projection();
            LineString(
                repr_3d
                    .into_iter()
                    .map(|p| proj.transform_point3(p))
                    .map(|p| p.truncate())
                    .collect::<Vec<_>>(),
            )
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            let inj = working_plane.xy_injection();
            self.points()
                .map(|vec2| vec2.extend(0.0))
                .map(|vec3| inj.transform_point3(vec3))
                .collect::<Vec<_>>()
        }
    }
    impl Flattenable for MultiLineString {
        type Type3D = Vec<Vec<Vec3>>;
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            MultiLineString(
                repr_3d
                    .into_iter()
                    .map(|linestring| LineString::embed(linestring, working_plane))
                    .collect::<Vec<_>>(),
            )
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            self.0
                .into_iter()
                .map(|linestring| linestring.unembed(working_plane))
                .collect::<Vec<_>>()
        }
    }
    impl Flattenable for Ring {
        type Type3D = Vec<Vec3>;
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let proj = working_plane.xy_projection();
            Ring::new(
                repr_3d
                    .into_iter()
                    .map(|vec3| proj.transform_point3(vec3))
                    .map(|vec2| vec2.truncate())
                    .collect::<Vec<_>>(),
            )
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            let inj = working_plane.xy_injection();
            self.to_linestring()
                .points()
                .map(|vec2| vec2.extend(0.0))
                .map(|vec3| inj.transform_point3(vec3))
                .collect::<Vec<_>>()
        }
    }
    impl Flattenable for MultiRing {
        type Type3D = Vec<Vec<Vec3>>;
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            MultiRing(
                repr_3d
                    .into_iter()
                    .map(|ring| Ring::embed(ring, working_plane))
                    .collect::<Vec<_>>(),
            )
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            self.0
                .into_iter()
                .map(|ring| ring.unembed(working_plane))
                .collect::<Vec<_>>()
        }
    }
    impl Flattenable for Polygon {
        type Type3D = (Vec<Vec3>, Vec<Vec<Vec3>>);
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let (ring, multiring) = (
                Ring::embed(repr_3d.0, working_plane),
                MultiRing::embed(repr_3d.1, working_plane),
            );
            Polygon::new(ring, multiring)
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            let (ring, multiring) = (self.0, self.1);
            (
                ring.unembed(working_plane),
                multiring.unembed(working_plane),
            )
        }
    }
    impl Flattenable for MultiPolygon {
        type Type3D = Vec<(Vec<Vec3>, Vec<Vec<Vec3>>)>;
        #[inline]
        fn embed(repr_3d: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            MultiPolygon(
                repr_3d
                    .into_iter()
                    .map(|polygon| Polygon::embed(polygon, working_plane))
                    .collect::<Vec<_>>(),
            )
        }
        #[inline]
        fn unembed(self, working_plane: crate::prelude::WorkingPlane) -> Self::Type3D {
            self.0
                .into_iter()
                .map(|polygon| polygon.unembed(working_plane))
                .collect::<Vec<_>>()
        }
    }
}
