use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;
use math::stitch_triangles_glam;

use crate::{
    spawner::SpawnPolygon,
    triangle::{Triangle, TriangleLine, TriangleParams, TrianglePoint},
};

use super::algostate::AlgorithmState;

pub struct StitchTrianglesPlugin;

impl Plugin for StitchTrianglesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                render_polygon_stitch,
                do_stitching.run_if(input_just_pressed(KeyCode::Enter)),
            )
                .run_if(in_state(AlgorithmState::StitchTriangles)),
        );
    }
}

fn render_polygon_stitch(mut gizmos: Gizmos, triangles: TriangleParams) {
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

fn do_stitching(
    mut cmds: Commands,
    mut spawn_polygons: EventWriter<SpawnPolygon>,
    triangles: TriangleParams,
    entities: Query<Entity, Or<(With<Triangle>, With<TriangleLine>, With<TrianglePoint>)>>,
) {
    spawn_polygons.send_batch(
        triangles
            .iter_triangles()
            .chunk_by(|(_, wp)| *wp)
            .into_iter()
            .flat_map(|(wp, group)| {
                let (proj, inj) = wp.xy_projection_injection();
                let triangles_projected = group
                    .into_iter()
                    .map(|(triangle, _)| triangle)
                    .map(|triangle| triangle.map(|p| proj.transform_point(p).truncate()))
                    .collect::<Vec<_>>();
                stitch_triangles_glam(triangles_projected)
                    .into_iter()
                    .map(move |polygon| {
                        let points = polygon
                            .into_iter()
                            .map(|start| inj.transform_point(start.extend(0.0)))
                            .collect::<Vec<_>>();
                        SpawnPolygon { points }
                    })
            }),
    );

    entities.iter().for_each(|entity| {
        cmds.entity(entity).despawn_recursive();
    });
}
