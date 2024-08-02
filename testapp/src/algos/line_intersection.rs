use bevy::{color::palettes, prelude::*};

use crate::{line::LineParams, state::AlgoState};

pub struct LineIntersectionPlugin;

impl Plugin for LineIntersectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            render_intersection_points.run_if(in_state(AlgoState::LineIntersection)),
        );
    }
}

fn render_intersection_points(mut gizmos: Gizmos, lines: LineParams) {
    lines
        .iter_lines()
        .enumerate()
        .flat_map(|(i, line_a)| {
            lines
                .iter_lines()
                .skip(i + 1)
                .map(move |line_b| (line_a, line_b))
        })
        .filter_map(|([a, b], [c, d])| math::intersect_line_2d_point((a, b), (c, d)))
        .for_each(|intersection| {
            gizmos.circle_2d(intersection, 5.0, palettes::basic::RED);
        });
}
