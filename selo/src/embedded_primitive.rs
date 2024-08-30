use glam::{Vec2, Vec3};

use crate::{prelude::WorkingPlane, Map};

/// A trait to classify 2D geometric types that can be created from points on a 3D working plane and
/// the plane itself
pub trait Unembed {
    /// Representation of the flat geometry in 3D coordinates
    type Type3D: Embed<Type2D = Self>;

    /// method to transform the geometry from the XY plane with 2D coordinates back to a 3D plane
    fn unembed(&self, working_plane: WorkingPlane) -> Self::Type3D;
}

pub trait Embed {
    /// Flattened representation of this 3D geometry
    type Type2D;

    /// method to transform the geometry from a 3D plane to the XY plane into 2D coordinates
    fn embed(&self, workplane: WorkingPlane) -> Self::Type2D;
}

/// This type represents geometry in a 3D context which was projected to 2D coordinates to apply
/// algorithms for problems which are typically easier to solve in 2D space.
///
/// ```
/// # use selo::prelude::*;
///
/// let [a,b,c] = [Vec3::X, Vec3::Y, Vec3::Z];
/// let plane = WorkingPlane::from_three_points([a,b,c]);
///
/// let triangle_2d = FlatPrimitive::<Triangle<Vec2>>::new(Triangle([a,b,c]), plane);
/// ```
#[derive(Debug, Clone)]
pub struct FlatPrimitive<P: Unembed> {
    primitive: P,
    working_plane: WorkingPlane,
}

impl<A: Unembed> FlatPrimitive<A> {
    /// Transforms a given 3D geometry that is flat with respect to some [`WorkingPlane`] into 2D space
    ///
    /// ```
    /// # use selo::prelude::*;
    /// # use glam::Vec3;
    ///
    /// let [a,b,c] = [Vec3::X, Vec3::Y, Vec3::Z];
    /// let plane = WorkingPlane::from_three_points([a,b,c]);
    ///
    /// let triangle_2d = FlatPrimitive::<Triangle<Vec2>>::new(Triangle([a,b,c]), plane);
    /// ```
    pub fn new(from: A::Type3D, working_plane: WorkingPlane) -> Self {
        Self {
            primitive: from.embed(working_plane),
            working_plane,
        }
    }

    /// Apply transformations to the flattened 2D geometry
    ///
    /// ```
    /// # use selo::prelude::*;
    ///
    /// let [a,b,c] = [Vec3::X, Vec3::Y, Vec3::Z];
    /// let plane = WorkingPlane::from_three_points([a,b,c]);
    ///
    /// let triangle_2d = FlatPrimitive::<Triangle<Vec2>>::new(Triangle([a,b,c]), plane);
    ///
    /// let flip_triangle = |triangle: Triangle<Vec2>| -> Triangle<Vec2> {
    ///     Triangle(triangle.0.map(|mut vec2| {
    ///         vec2.y = -vec2.y;
    ///         vec2
    ///     }))
    /// };
    ///
    /// let flipped_triangle = triangle_2d.map_geometry(flip_triangle);
    /// ```
    pub fn map_geometry<B: Unembed>(self, f: impl Fn(A) -> B) -> FlatPrimitive<B> {
        FlatPrimitive {
            primitive: f(self.primitive),
            working_plane: self.working_plane,
        }
    }

    /// Transform the 2D geometry back into 3D space onto the [`WorkingPlane`] where it came from.
    ///
    /// ```
    /// # use selo::prelude::*;
    /// # use glam::Vec3;
    ///
    /// let [a,b,c] = [Vec3::X, Vec3::Y, Vec3::Z];
    /// let plane = WorkingPlane::from_three_points([a,b,c]);
    ///
    /// let triangle_2d = FlatPrimitive::<Triangle<Vec2>>::new(Triangle([a,b,c]), plane);
    ///
    /// let flip_triangle = |triangle: Triangle<Vec2>| -> Triangle<Vec2> {
    ///     Triangle(triangle.0.map(|mut vec2| {
    ///         vec2.y = -vec2.y;
    ///         vec2
    ///     }))
    /// };
    ///
    /// let flipped_triangle = triangle_2d.map_geometry(flip_triangle);
    ///
    /// let (Triangle([a,b,c]), plane) = flipped_triangle.unpack();
    /// ```
    pub fn unpack(self) -> (A::Type3D, WorkingPlane) {
        (
            A::unembed(&self.primitive, self.working_plane),
            self.working_plane,
        )
    }
}

impl<T: Map<Vec2, Vec3>> Unembed for T
where
    T::Output: Embed<Type2D = T>,
{
    type Type3D = T::Output;

    fn unembed(&self, working_plane: WorkingPlane) -> Self::Type3D {
        let inj = working_plane.xy_injection();
        self.map(|p| inj.transform_point3(p.extend(0.0)))
    }
}

impl<T: Map<Vec3, Vec2>> Embed for T {
    type Type2D = T::Output;

    fn embed(&self, working_plane: WorkingPlane) -> Self::Type2D {
        let proj = working_plane.xy_projection();
        self.map(|p| proj.transform_point3(p).truncate())
    }
}
