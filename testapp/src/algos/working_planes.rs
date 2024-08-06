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

    let transform = plane.xy_injection();

    let triangle = [Vec2::X, Vec2::Y, Vec2::ONE].map(|p| p.extend(0.0));
    let triangle_in_plane = triangle.map(|p| transform.transform_point3(p));

    gizmos.sphere(plane.origin, Quat::default(), 0.025, palettes::basic::BLUE);
    gizmos.sphere(
        normalized_plane.origin,
        Quat::default(),
        0.025,
        palettes::basic::GREEN,
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
            gizmos.line(a, b, palettes::basic::AQUA);
        });

    gizmos.line(
        plane.origin,
        normalized_plane.origin,
        palettes::basic::WHITE,
    );
    gizmos.line(plane.origin, Vec3::ZERO, palettes::basic::WHITE);
    gizmos.line(normalized_plane.origin, Vec3::ZERO, palettes::basic::WHITE);

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

fn ui(world: &mut World) {
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    egui::Window::new("Working Plane Points").show(&ctx, |ui| {
        ui_for_resource::<WorkingPlanePoints>(world, ui);
    });
}
