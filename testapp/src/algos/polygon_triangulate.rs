use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;
use selo::{triangulate_glam, Embed, Ring, Unembed};

use crate::{
    ring::{Ring2D, RingLine, RingParams, RingPoint},
    spawner::SpawnTriangle,
};

use super::algostate::AlgorithmState;

pub struct PolygonTriangulationPlugin;

impl Plugin for PolygonTriangulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                render_triangulation,
                do_triangulation.run_if(input_just_pressed(KeyCode::Enter)),
            )
                .run_if(in_state(AlgorithmState::PolygonTriangulate)),
        );
    }
}

fn render_triangulation(mut gizmos: Gizmos, rings: RingParams) {
    rings
        .iter_rings()
        .chunk_by(|(_, wp)| *wp)
        .into_iter()
        .for_each(|(wp, group)| {
            group
                .into_iter()
                .flat_map(|(ring, _)| wp.transform(ring.to_polygon(), triangulate_glam))
                .for_each(|selo::Triangle([a, b, c])| {
                    gizmos.primitive_3d(
                        &Triangle3d::new(a, b, c),
                        Isometry3d::new(Vec3::ZERO, Quat::default()),
                        palettes::basic::RED,
                    );
                });
        });
}

fn do_triangulation(
    mut cmds: Commands,
    mut spawn_triangles: EventWriter<SpawnTriangle>,
    rings: RingParams,
    entities: Query<Entity, Or<(With<Ring2D>, With<RingLine>, With<RingPoint>)>>,
) {
    spawn_triangles.write_batch(
        rings
            .iter_rings()
            .chunk_by(|(_, wp)| *wp)
            .into_iter()
            .flat_map(|(wp, group)| {
                group
                    .into_iter()
                    .map(move |(ring, _)| Ring::embed(&ring, wp))
                    .flat_map(|ring| triangulate_glam(ring.to_polygon()))
                    .map(move |tri| tri.unembed(wp))
                    .map(|tri| SpawnTriangle(tri))
            }),
    );

    entities.iter().for_each(|entity| {
        cmds.entity(entity).despawn();
    });
}
