use bevy::{color::palettes, prelude::*};

use crate::{line::LineParams, state::AlgorithmState};

pub struct LineIntersectionPlugin;

impl Plugin for LineIntersectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            render_intersection_points.run_if(in_state(AlgorithmState::LineIntersection)),
        );
    }
}

fn render_intersection_points(mut gizmos: Gizmos, lines: LineParams) {
    lines
        .iter_lines()
        // don't do that anymore and instead rotate to plane
        .map(|line| line.map(|p| p.truncate()))
        .enumerate()
        .flat_map(|(i, line_a)| {
            lines
                .iter_lines()
                // don't do that anymore and instead rotate to plane
                .map(|line| line.map(|p| p.truncate()))
                .skip(i + 1)
                .map(move |line_b| (line_a, line_b))
        })
        .filter_map(|([a, b], [c, d])| math::intersect_line_2d_point((a, b), (c, d)))
        .for_each(|intersection| {
            gizmos.circle_2d(intersection, 5.0, palettes::basic::RED);
        });
}
