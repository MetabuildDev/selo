use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;
use selo::{stitch_triangles_glam, Embed, Unembed};

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
                .map(|triangle| triangle.embed(wp))
                .collect::<Vec<_>>();
            stitch_triangles_glam(triangles_projected)
                .0
                .into_iter()
                .for_each(|mp| {
                    mp.unembed(wp).lines().for_each(|line| {
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
    spawn_rings.write_batch(
        triangles
            .iter_triangles()
            .chunk_by(|(_, wp)| *wp)
            .into_iter()
            .flat_map(|(wp, group)| {
                let triangles_projected = group
                    .into_iter()
                    .map(|(triangle, _)| triangle)
                    .map(|triangle| triangle.embed(wp))
                    .collect::<Vec<_>>();
                stitch_triangles_glam(triangles_projected)
                    .0
                    .into_iter()
                    .map(move |mp| SpawnRing(mp.exterior().unembed(wp)))
            }),
    );

    entities.iter().for_each(|entity| {
        cmds.entity(entity).despawn();
    });
}
