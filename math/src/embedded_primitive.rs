use crate::prelude::WorkingPlane;

/// A trait to classify 2D geometric types that can be created from points on a 3D working plane and
/// the plane itself
pub trait Embeddable {
    /// Representation of the embeddable type in 3D
    type Type3D;

    /// method to transform the geometry from the 3D plane to the XY plane into 2D coordinates
    fn embed(from: Self::Type3D, working_plane: WorkingPlane) -> Self;
    /// method to transform the geometry from the XY plane with 2D coordinates back to the 3D plane
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
pub struct EmbeddedPrimitive<P: Embeddable> {
    primitive: P,
    working_plane: WorkingPlane,
}

impl<A: Embeddable> EmbeddedPrimitive<A> {
    /// Embedds a given 3D representation of a type that is embeddable into 2D space
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

    /// Apply transformations to the embedded 2D geometry
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
    pub fn map_geometry<B: Embeddable>(self, f: impl Fn(A) -> B) -> EmbeddedPrimitive<B> {
        EmbeddedPrimitive {
            primitive: f(self.primitive),
            working_plane: self.working_plane,
        }
    }

    /// Unembed the 2D geometry back into 3D space onto the working plane where it came from.
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
    use super::Embeddable;
    use crate::primitives::*;
    use glam::*;

    impl Embeddable for Line {
        type Type3D = [Vec3; 2];
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let proj = working_plane.xy_projection();
            Line(
                from.map(|vec3| proj.transform_point3(vec3))
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
    impl Embeddable for Triangle {
        type Type3D = [Vec3; 3];
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let proj = working_plane.xy_projection();
            Triangle(
                from.map(|vec3| proj.transform_point3(vec3))
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
    impl Embeddable for MultiTriangle {
        type Type3D = Vec<[Vec3; 3]>;
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            MultiTriangle(
                from.into_iter()
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
    impl Embeddable for LineString {
        type Type3D = Vec<Vec3>;
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let proj = working_plane.xy_projection();
            LineString(
                from.into_iter()
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
    impl Embeddable for MultiLineString {
        type Type3D = Vec<Vec<Vec3>>;
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            MultiLineString(
                from.into_iter()
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
    impl Embeddable for Ring {
        type Type3D = Vec<Vec3>;
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let proj = working_plane.xy_projection();
            Ring::new(
                from.into_iter()
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
    impl Embeddable for MultiRing {
        type Type3D = Vec<Vec<Vec3>>;
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            MultiRing(
                from.into_iter()
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
    impl Embeddable for Polygon {
        type Type3D = (Vec<Vec3>, Vec<Vec<Vec3>>);
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            let (ring, multiring) = (
                Ring::embed(from.0, working_plane),
                MultiRing::embed(from.1, working_plane),
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
    impl Embeddable for MultiPolygon {
        type Type3D = Vec<(Vec<Vec3>, Vec<Vec<Vec3>>)>;
        #[inline]
        fn embed(from: Self::Type3D, working_plane: crate::prelude::WorkingPlane) -> Self {
            MultiPolygon(
                from.into_iter()
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
