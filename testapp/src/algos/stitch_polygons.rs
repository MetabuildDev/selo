use bevy::{color::palettes, prelude::*};
use itertools::Itertools;
use math::{primitives::Triangle, stitch_triangles_glam};

use crate::triangle::TriangleParams;

use super::algostate::AlgorithmState;

pub struct StitchTrianglesPlugin;

impl Plugin for StitchTrianglesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (render_polygon_expansion,).run_if(in_state(AlgorithmState::StitchTriangles)),
        );
    }
}

fn render_polygon_expansion(mut gizmos: Gizmos, triangles: TriangleParams) {
    triangles
        .iter_triangles()
        .chunk_by(|(_, wp)| *wp)
        .into_iter()
        .for_each(|(wp, group)| {
            let (proj, inj) = wp.xy_projection_injection();
            let triangles_projected = group
                .into_iter()
                .map(|(triangle, _)| triangle)
                .map(|triangle| Triangle(triangle.map(|p| proj.transform_point(p).truncate())))
                .collect::<Vec<_>>();
            stitch_triangles_glam(triangles_projected)
                .into_iter()
                .for_each(|polygon| {
                    polygon
                        .lines()
                        .map(|line| line.0.map(|p| inj.transform_point(p.extend(0.0))))
                        .for_each(|line| {
                            gizmos.line(line[0], line[1], palettes::basic::RED);
                        });
                });
        })
}
