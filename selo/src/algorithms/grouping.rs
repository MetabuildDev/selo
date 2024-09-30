use bevy_math::Vec3;

use crate::{prelude::Workplane, Embed, IterPoints, Normal};

/// Group primitives based on their plane
pub fn group_primitives<T: IterPoints<P = Vec3> + Normal<P = Vec3> + Embed>(
    p: impl IntoIterator<Item = T>,
    tolerance: f32,
) -> Vec<(Workplane, Vec<T::Type2D>)> {
    let mut groups: Vec<(Workplane, Vec<T::Type2D>)> = vec![];
    for p in p {
        let Ok(wp) = Workplane::from_primitive(&p) else {
            continue;
        };
        let wp = wp.hesse_normal_form();
        if let Some((wp, group)) = groups.iter_mut().find(|(other, _)| {
            wp.plane.normal.abs_diff_eq(*other.plane.normal, tolerance)
                && wp.origin.abs_diff_eq(other.origin, tolerance)
        }) {
            group.push(p.embed(*wp));
        } else {
            groups.push((wp, vec![p.embed(wp)]))
        }
    }
    groups
}
