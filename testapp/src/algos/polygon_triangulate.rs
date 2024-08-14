use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;
use math::{primitives::Ring, triangulate_glam};

use crate::{line::Line, polygon::PolygonParams, spawner::SpawnTriangle};

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

fn render_triangulation(mut gizmos: Gizmos, polygons: PolygonParams) {
    polygons
        .iter_polygons()
        .chunk_by(|(_, wp)| *wp)
        .into_iter()
        .for_each(|(wp, group)| {
            let (proj, inj) = wp.xy_projection_injection();
            group
                .into_iter()
                .map(|(poly, _)| poly)
                .map(|polygon| {
                    Ring::new(
                        polygon
                            .into_iter()
                            .map(|p| proj.transform_point(p).truncate())
                            .collect::<Vec<_>>(),
                    )
                })
                .flat_map(|ring| triangulate_glam(ring.to_polygon()))
                .map(|tri| tri.0.map(|p| inj.transform_point(p.extend(0.0))))
                .for_each(|[a, b, c]| {
                    gizmos.primitive_3d(
                        &Triangle3d::new(a, b, c),
                        Vec3::ZERO,
                        Quat::default(),
                        palettes::basic::RED,
                    );
                });
        });
}

fn do_triangulation(
    mut cmds: Commands,
    mut spawn_triangles: EventWriter<SpawnTriangle>,
    polygons: PolygonParams,
    lines: Query<&Line>,
) {
    spawn_triangles.send_batch(
        polygons
            .iter_polygons()
            .chunk_by(|(_, wp)| *wp)
            .into_iter()
            .flat_map(|(wp, group)| {
                let (proj, inj) = wp.xy_projection_injection();
                group
                    .into_iter()
                    .map(|(poly, _)| poly)
                    .map(move |polygon| {
                        Ring::new(
                            polygon
                                .into_iter()
                                .map(|p| proj.transform_point(p).truncate())
                                .collect::<Vec<_>>(),
                        )
                    })
                    .flat_map(|ring| triangulate_glam(ring.to_polygon()))
                    .map(move |tri| tri.0.map(|p| inj.transform_point(p.extend(0.0))))
                    .map(|[a, b, c]| SpawnTriangle { a, b, c })
            }),
    );

    polygons.iter_entities().for_each(|(poly, lines_vec)| {
        cmds.entity(poly).despawn_recursive();
        lines_vec.iter().for_each(|line| {
            cmds.entity(*line).despawn_recursive();
            if let Ok(Line { start, end }) = lines.get(*line) {
                cmds.entity(*start).despawn_recursive();
                cmds.entity(*end).despawn_recursive();
            }
        });
    });
}
