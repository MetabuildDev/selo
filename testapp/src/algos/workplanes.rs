use crate::gizmos::GizmosExt;
use bevy::{color::palettes, prelude::*};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::ui_for_resource;
use selo::prelude::Workplane;

use crate::gizmos::AnimatedGizmos;

use super::algostate::AlgorithmState;

pub struct WorkplanePlugin;

impl Plugin for WorkplanePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WorkplanePoints>()
            .add_systems(
                OnEnter(AlgorithmState::WorkplaneNormalization),
                |mut cmds: Commands| {
                    cmds.init_resource::<WorkplanePoints>();
                },
            )
            .add_systems(
                OnExit(AlgorithmState::WorkplaneNormalization),
                |mut cmds: Commands| {
                    cmds.remove_resource::<WorkplanePoints>();
                },
            )
            .add_systems(
                OnEnter(AlgorithmState::WorkplaneTransform),
                |mut cmds: Commands| {
                    cmds.init_resource::<WorkplanePoints>();
                },
            )
            .add_systems(
                OnExit(AlgorithmState::WorkplaneTransform),
                |mut cmds: Commands| {
                    cmds.remove_resource::<WorkplanePoints>();
                },
            )
            .add_systems(
                Update,
                (
                    render_workplane_normalization,
                    ui.run_if(resource_exists::<WorkplanePoints>),
                )
                    .run_if(in_state(AlgorithmState::WorkplaneNormalization)),
            )
            .add_systems(
                Update,
                (
                    render_workplane_transform,
                    ui.run_if(resource_exists::<WorkplanePoints>),
                )
                    .run_if(in_state(AlgorithmState::WorkplaneTransform)),
            );
    }
}

#[derive(Debug, Clone, Resource, Deref, DerefMut, Reflect)]
struct WorkplanePoints([Vec3; 3]);

impl Default for WorkplanePoints {
    fn default() -> Self {
        Self([Vec3::ZERO, Vec3::X, Vec3::Y].map(|p| p + Vec3::Z))
    }
}

fn render_workplane_normalization(mut gizmos: AnimatedGizmos, points: Res<WorkplanePoints>) {
    let plane = Workplane::from_three_points(**points);
    let normalized_plane = plane.hesse_normal_form();

    gizmos.sphere(plane.origin, 0.025, palettes::basic::BLUE);
    gizmos.sphere(normalized_plane.origin, 0.025, palettes::basic::YELLOW);

    [
        (plane.origin, normalized_plane.origin),
        (plane.origin, Vec3::ZERO),
        (normalized_plane.origin, Vec3::ZERO),
    ]
    .into_iter()
    .for_each(|(start, end)| {
        gizmos.animated_line(start, end, palettes::basic::WHITE, 0.2, 10);
    });

    gizmos.plane_3d(plane.origin, plane.normal(), palettes::basic::BLUE);
    gizmos.plane_3d(
        normalized_plane.origin,
        normalized_plane.normal(),
        palettes::basic::YELLOW,
    );
}

fn render_workplane_transform(mut gizmos: AnimatedGizmos, points: Res<WorkplanePoints>) {
    let plane = Workplane::from_three_points(**points);
    let normalized_plane = plane.hesse_normal_form();

    let transform = plane.xy_injection();

    let triangle = [Vec2::X, Vec2::Y, Vec2::ONE].map(|p| p.extend(0.0));
    let triangle_in_plane = triangle.map(|p| transform.transform_point3(p));

    gizmos.sphere(normalized_plane.origin, 0.025, palettes::basic::YELLOW);

    triangle
        .windows(2)
        .map(|win| [win[0], win[1]])
        .chain(Some(()).and_then(|_| {
            let first = triangle.first()?;
            let last = triangle.last()?;
            Some([*first, *last])
        }))
        .for_each(|[a, b]| {
            gizmos.line(a, b, palettes::basic::GREEN);
        });

    triangle_in_plane
        .windows(2)
        .map(|win| [win[0], win[1]])
        .chain(Some(()).and_then(|_| {
            let first = triangle_in_plane.first()?;
            let last = triangle_in_plane.last()?;
            Some([*first, *last])
        }))
        .for_each(|[a, b]| {
            gizmos.line(a, b, palettes::basic::RED);
        });

    triangle
        .into_iter()
        .zip(triangle_in_plane)
        .for_each(|(a, b)| {
            gizmos.animated_line(a, b, palettes::basic::AQUA, 0.1, 10);
        });

    gizmos.plane_3d(
        normalized_plane.origin,
        normalized_plane.normal(),
        palettes::basic::YELLOW,
    );
}

fn ui(world: &mut World) {
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    egui::Window::new("Workplane Points").show(&ctx, |ui| {
        ui_for_resource::<WorkplanePoints>(world, ui);
    });
}
