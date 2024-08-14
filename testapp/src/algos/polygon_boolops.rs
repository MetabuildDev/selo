use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;
use math::boolops_union_glam;

use crate::{
    polygon::{Polygon2D, PolygonLine, PolygonParams, PolygonPoint},
    spawner::SpawnPolygon,
};

use super::algostate::AlgorithmState;

pub struct PolygonBoolopsPlugin;

impl Plugin for PolygonBoolopsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                render_polygon_union,
                do_unioning.run_if(input_just_pressed(KeyCode::Enter)),
            )
                .run_if(in_state(AlgorithmState::PolygonBoolops)),
        );
    }
}

fn render_polygon_union(mut gizmos: Gizmos, polygons: PolygonParams) {
    polygons
        .iter_polygons()
        .chunk_by(|(_, wp)| *wp)
        .into_iter()
        .for_each(|(wp, group)| {
            let (proj, inj) = wp.xy_projection_injection();
            let polygons_projected = group
                .into_iter()
                .map(|(poly, _)| poly)
                .map(|polygon| {
                    polygon
                        .into_iter()
                        .map(|p| proj.transform_point(p).truncate())
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>();
            boolops_union_glam(polygons_projected)
                .into_iter()
                .for_each(|(exterior, interiors)| {
                    let mut render = |linestring: &[Vec2]| {
                        linestring
                            .windows(2)
                            .map(|win| (win[0], win[1]))
                            .chain(Some(()).and_then(|_| {
                                let first = linestring.first()?;
                                let last = linestring.last()?;
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
                    };
                    render(&exterior);
                    interiors.iter().for_each(|interior| {
                        render(&interior);
                    });
                });
        });
}

fn do_unioning(
    mut cmds: Commands,
    mut spawn_polygons: EventWriter<SpawnPolygon>,
    polygons: PolygonParams,
    entities: Query<Entity, Or<(With<Polygon2D>, With<PolygonLine>, With<PolygonPoint>)>>,
) {
    spawn_polygons.send_batch(
        polygons
            .iter_polygons()
            .chunk_by(|(_, wp)| *wp)
            .into_iter()
            .flat_map(|(wp, group)| {
                let (proj, inj) = wp.xy_projection_injection();
                let polygons_projected = group
                    .into_iter()
                    .map(|(polygon, _)| polygon)
                    .map(|polygon| {
                        polygon
                            .into_iter()
                            .map(|p| proj.transform_point(p).truncate())
                            .collect::<Vec<_>>()
                    })
                    .collect::<Vec<_>>();
                boolops_union_glam(polygons_projected).into_iter().map(
                    move |(exterior, _interiors)| {
                        let points = exterior
                            .into_iter()
                            .map(|start| inj.transform_point(start.extend(0.0)))
                            .collect::<Vec<_>>();
                        SpawnPolygon { points }
                    },
                )
            }),
    );

    entities.iter().for_each(|entity| {
        cmds.entity(entity).despawn_recursive();
    });
}
