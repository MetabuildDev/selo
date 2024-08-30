use bevy::{color::palettes, prelude::*};
use itertools::Itertools;
use selo::prelude::*;

use crate::line::LineParams;

use super::algostate::AlgorithmState;

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
        .chunk_by(|(_, wp)| *wp)
        .into_iter()
        .for_each(|(wp, group)| {
            let inj = wp.xy_injection();
            let group = group.map(|(line, _)| line).collect::<Vec<_>>();
            group
                .iter()
                .map(|line| line.embed(wp))
                .tuple_combinations()
                .filter_map(|(a, b)| selo::intersect_line_2d_point(a, b))
                .for_each(|intersection| {
                    gizmos.sphere(
                        inj.transform_point(intersection.extend(0.0)),
                        Quat::default(),
                        0.025,
                        palettes::basic::RED,
                    );
                });
        });
}
