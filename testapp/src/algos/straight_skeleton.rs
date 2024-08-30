use bevy::{color::palettes, prelude::*};
use bevy_egui::{egui, EguiContext};
use itertools::Itertools;
use selo::{skeleton_lines_glam, Flattenable as _, Ring};

use crate::ring::RingParams;

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
                    render_straight_skeleton,
                    ui.run_if(resource_exists::<SkeletonOrientation>),
                )
                    .run_if(in_state(AlgorithmState::StraightSkeleton)),
            );
    }
}

#[derive(Debug, Clone, Resource, Deref, DerefMut, Reflect, Default)]
struct SkeletonOrientation(bool);

fn render_straight_skeleton(
    mut gizmos: Gizmos,
    rings: RingParams,
    orientation: Res<SkeletonOrientation>,
) {
    rings
        .iter_rings()
        .chunk_by(|(_, wp)| *wp)
        .into_iter()
        .for_each(|(wp, group)| {
            group
                .into_iter()
                .map(|(ring, _)| Ring::embed(&ring, wp))
                .flat_map(|ring| skeleton_lines_glam(ring.to_polygon(), !**orientation))
                .for_each(|polygon| {
                    polygon.unembed(wp).lines().for_each(|line| {
                        gizmos.line(line.src(), line.dst(), palettes::basic::RED);
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
