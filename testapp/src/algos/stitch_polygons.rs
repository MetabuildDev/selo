use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;
use selo::{stitch_triangles_glam, Flattenable};

use crate::{
    spawner::SpawnRing,
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
            let triangles_projected = group
                .into_iter()
                .map(|(triangle, _)| triangle)
                .map(|triangle| selo::Triangle::embed(&triangle, wp))
                .collect::<Vec<_>>();
            stitch_triangles_glam(triangles_projected)
                .into_iter()
                .for_each(|ring| {
                    ring.unembed(wp).lines().for_each(|line| {
                        gizmos.line(line.src(), line.dst(), palettes::basic::RED);
                    });
                });
        });
}

fn do_stitching(
    mut cmds: Commands,
    mut spawn_rings: EventWriter<SpawnRing>,
    triangles: TriangleParams,
    entities: Query<Entity, Or<(With<Triangle>, With<TriangleLine>, With<TrianglePoint>)>>,
) {
    spawn_rings.send_batch(
        triangles
            .iter_triangles()
            .chunk_by(|(_, wp)| *wp)
            .into_iter()
            .flat_map(|(wp, group)| {
                let triangles_projected = group
                    .into_iter()
                    .map(|(triangle, _)| triangle)
                    .map(|triangle| selo::Triangle::embed(&triangle, wp))
                    .collect::<Vec<_>>();
                stitch_triangles_glam(triangles_projected)
                    .into_iter()
                    .map(move |ring| SpawnRing(ring.unembed(wp)))
            }),
    );

    entities.iter().for_each(|entity| {
        cmds.entity(entity).despawn_recursive();
    });
}
