use bevy::{color::palettes, prelude::*};
use itertools::Itertools;
use math::stitch_triangles_glam;

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
                .map(|triangle| triangle.map(|p| proj.transform_point(p).truncate()))
                .collect::<Vec<_>>();
            stitch_triangles_glam(triangles_projected)
                .into_iter()
                .for_each(|polygon| {
                    polygon
                        .windows(2)
                        .map(|win| (win[0], win[1]))
                        .chain(Some(()).and_then(|_| {
                            let first = polygon.first()?;
                            let last = polygon.last()?;
                            (first != last).then_some((*first, *last))
                        }))
                        .map(|(start, end)| {
                            (
                                inj.transform_point(start.extend(0.0)),
                                inj.transform_point(end.extend(0.0)),
                            )
                        })
                        .for_each(|(start, end)| {
                            gizmos.line(start, end, palettes::basic::RED);
                        });
                });
        })
}
