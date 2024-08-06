use bevy::{color::palettes, prelude::*};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::ui_for_resource;
use math::prelude::WorkingPlane;

use super::algostate::AlgorithmState;

pub struct WorkingPlanePlugin;

impl Plugin for WorkingPlanePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WorkingPlanePoints>()
            .add_systems(
                OnEnter(AlgorithmState::WorkingPlaneShowcase),
                |mut cmds: Commands| {
                    cmds.init_resource::<WorkingPlanePoints>();
                },
            )
            .add_systems(
                OnExit(AlgorithmState::WorkingPlaneShowcase),
                |mut cmds: Commands| {
                    cmds.remove_resource::<WorkingPlanePoints>();
                },
            )
            .add_systems(
                Update,
                (
                    render_working_plane_center,
                    ui.run_if(resource_exists::<WorkingPlanePoints>),
                )
                    .run_if(in_state(AlgorithmState::WorkingPlaneShowcase)),
            );
    }
}

#[derive(Debug, Clone, Resource, Deref, DerefMut, Reflect)]
struct WorkingPlanePoints(Vec<Vec3>);

impl Default for WorkingPlanePoints {
    fn default() -> Self {
        Self([Vec3::ZERO, Vec3::X, Vec3::Y].map(|p| p + Vec3::Z).to_vec())
    }
}

fn render_working_plane_center(mut gizmos: Gizmos, points: Res<WorkingPlanePoints>) {
    let plane = WorkingPlane::from_points(points.iter().copied());
    let normalized_plane = plane.normalize();

    gizmos.sphere(plane.origin, Quat::default(), 0.025, palettes::basic::RED);
    gizmos.sphere(
        normalized_plane.origin,
        Quat::default(),
        0.025,
        palettes::basic::RED,
    );
    gizmos.arrow(plane.origin, normalized_plane.origin, palettes::basic::RED);
    gizmos.line(plane.origin, Vec3::ZERO, palettes::basic::RED);
    gizmos.line(normalized_plane.origin, Vec3::ZERO, palettes::basic::RED);
    gizmos
        .primitive_3d(
            &Plane3d {
                normal: plane.plane.normal,
                half_size: Vec2::new(1000.0, 1000.0),
            },
            plane.origin,
            Quat::default(),
            palettes::basic::RED,
        )
        .segment_count(100)
        .segment_length(0.1);
    gizmos
        .primitive_3d(
            &Plane3d {
                normal: normalized_plane.plane.normal,
                half_size: Vec2::new(100.0, 100.0),
            },
            normalized_plane.origin,
            Quat::default(),
            palettes::basic::RED,
        )
        .segment_count(100)
        .segment_length(0.1);
}

fn ui(world: &mut World) {
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    egui::Window::new("Working Plane Points").show(&ctx, |ui| {
        ui_for_resource::<WorkingPlanePoints>(world, ui);
    });
}
