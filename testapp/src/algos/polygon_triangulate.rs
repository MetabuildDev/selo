use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;
use math::triangulate_glam;

use crate::{
    polygon::{Polygon2D, PolygonLine, PolygonParams, PolygonPoint},
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
                    polygon
                        .into_iter()
                        .map(|p| proj.transform_point(p).truncate())
                        .collect::<Vec<_>>()
                })
                .flat_map(|polygon| triangulate_glam(polygon))
                .map(|points| points.map(|p| inj.transform_point(p.extend(0.0))))
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
    entities: Query<Entity, Or<(With<Polygon2D>, With<PolygonLine>, With<PolygonPoint>)>>,
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
                        polygon
                            .into_iter()
                            .map(|p| proj.transform_point(p).truncate())
                            .collect::<Vec<_>>()
                    })
                    .flat_map(|polygon| triangulate_glam(polygon))
                    .map(move |points| points.map(|p| inj.transform_point(p.extend(0.0))))
                    .map(|[a, b, c]| SpawnTriangle { a, b, c })
            }),
    );

    entities.iter().for_each(|entity| {
        cmds.entity(entity).despawn_recursive();
    });
}
