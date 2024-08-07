use bevy::{color::palettes, prelude::*};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::ui_for_resource;
use math::prelude::WorkingPlane;

use crate::gizmos::AnimatedGizmos;

use super::algostate::AlgorithmState;

pub struct WorkingPlanePlugin;

impl Plugin for WorkingPlanePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<WorkingPlanePoints>()
            .add_systems(
                OnEnter(AlgorithmState::WorkingPlaneNormalization),
                |mut cmds: Commands| {
                    cmds.init_resource::<WorkingPlanePoints>();
                },
            )
            .add_systems(
                OnExit(AlgorithmState::WorkingPlaneNormalization),
                |mut cmds: Commands| {
                    cmds.remove_resource::<WorkingPlanePoints>();
                },
            )
            .add_systems(
                OnEnter(AlgorithmState::WorkingPlaneTransform),
                |mut cmds: Commands| {
                    cmds.init_resource::<WorkingPlanePoints>();
                },
            )
            .add_systems(
                OnExit(AlgorithmState::WorkingPlaneTransform),
                |mut cmds: Commands| {
                    cmds.remove_resource::<WorkingPlanePoints>();
                },
            )
            .add_systems(
                Update,
                (
                    render_working_plane_normalization,
                    ui.run_if(resource_exists::<WorkingPlanePoints>),
                )
                    .run_if(in_state(AlgorithmState::WorkingPlaneNormalization)),
            )
            .add_systems(
                Update,
                (
                    render_working_plane_transform,
                    ui.run_if(resource_exists::<WorkingPlanePoints>),
                )
                    .run_if(in_state(AlgorithmState::WorkingPlaneTransform)),
            );
    }
}

#[derive(Debug, Clone, Resource, Deref, DerefMut, Reflect)]
struct WorkingPlanePoints([Vec3; 3]);

impl Default for WorkingPlanePoints {
    fn default() -> Self {
        Self([Vec3::ZERO, Vec3::X, Vec3::Y].map(|p| p + Vec3::Z))
    }
}

fn render_working_plane_normalization(mut gizmos: AnimatedGizmos, points: Res<WorkingPlanePoints>) {
    let plane = WorkingPlane::from_three_points(**points);
    let normalized_plane = plane.hesse_normal_form();

    gizmos.sphere(plane.origin, Quat::default(), 0.025, palettes::basic::BLUE);
    gizmos.sphere(
        normalized_plane.origin,
        Quat::default(),
        0.025,
        palettes::basic::YELLOW,
    );

    [
        (plane.origin, normalized_plane.origin),
        (plane.origin, Vec3::ZERO),
        (normalized_plane.origin, Vec3::ZERO),
    ]
    .into_iter()
    .for_each(|(start, end)| {
        gizmos.animated_line(start, end, palettes::basic::WHITE, 0.2, 10);
    });

    gizmos
        .primitive_3d(
            &Plane3d {
                normal: plane.plane.normal,
                half_size: Vec2::new(1000.0, 1000.0),
            },
            plane.origin,
            Quat::default(),
            palettes::basic::BLUE,
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
            palettes::basic::YELLOW,
        )
        .segment_count(100)
        .segment_length(0.1);
    gizmos
        .primitive_3d(
            &Plane3d {
                normal: Dir3::Z,
                half_size: Vec2::new(100.0, 100.0),
            },
            Vec3::ZERO,
            Quat::default(),
            palettes::basic::GREEN,
        )
        .segment_count(100)
        .segment_length(0.1);
}

fn render_working_plane_transform(mut gizmos: AnimatedGizmos, points: Res<WorkingPlanePoints>) {
    let plane = WorkingPlane::from_three_points(**points);
    let normalized_plane = plane.hesse_normal_form();

    let transform = plane.xy_injection();

    let triangle = [Vec2::X, Vec2::Y, Vec2::ONE].map(|p| p.extend(0.0));
    let triangle_in_plane = triangle.map(|p| transform.transform_point3(p));

    gizmos.sphere(
        normalized_plane.origin,
        Quat::default(),
        0.025,
        palettes::basic::YELLOW,
    );

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

    gizmos
        .primitive_3d(
            &Plane3d {
                normal: normalized_plane.plane.normal,
                half_size: Vec2::new(100.0, 100.0),
            },
            normalized_plane.origin,
            Quat::default(),
            palettes::basic::YELLOW,
        )
        .segment_count(100)
        .segment_length(0.1);
    gizmos
        .primitive_3d(
            &Plane3d {
                normal: Dir3::Z,
                half_size: Vec2::new(100.0, 100.0),
            },
            Vec3::ZERO,
            Quat::default(),
            palettes::basic::GREEN,
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
