use bevy::{color::palettes, prelude::*};
use bevy_egui::{egui, EguiContext};
use itertools::Itertools;
use math::skeleton_lines_glam;

use crate::polygon::PolygonParams;

use super::algostate::AlgorithmState;

pub struct PolygonSkeletonPlugin;

impl Plugin for PolygonSkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<SkeletonOrientation>()
            .add_systems(
                OnEnter(AlgorithmState::StraightSkeleton),
                |mut cmds: Commands| {
                    cmds.init_resource::<SkeletonOrientation>();
                },
            )
            .add_systems(
                OnExit(AlgorithmState::StraightSkeleton),
                |mut cmds: Commands| {
                    cmds.remove_resource::<SkeletonOrientation>();
                },
            )
            .add_systems(
                Update,
                (
                    render_polygon_expansion,
                    ui.run_if(resource_exists::<SkeletonOrientation>),
                )
                    .run_if(in_state(AlgorithmState::StraightSkeleton)),
            );
    }
}

#[derive(Debug, Clone, Resource, Deref, DerefMut, Reflect, Default)]
struct SkeletonOrientation(bool);

fn render_polygon_expansion(
    mut gizmos: Gizmos,
    polygons: PolygonParams,
    orientation: Res<SkeletonOrientation>,
) {
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
                .flat_map(|polygon| skeleton_lines_glam(polygon, !**orientation))
                .for_each(|polygon| {
                    polygon
                        .windows(2)
                        .map(|win| (win[0], win[1]))
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
        });
}

fn ui(mut contexts: Query<&mut EguiContext>, mut val: ResMut<SkeletonOrientation>) {
    egui::Window::new("Straight Skeleton Orientation").show(
        contexts.single_mut().get_mut(),
        |ui| {
            ui.add(egui::Checkbox::new(&mut **val, "Orientation"));
        },
    );
}
