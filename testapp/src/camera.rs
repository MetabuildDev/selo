use bevy::{
    ecs::system::SystemParam,
    input::{
        common_conditions::input_pressed,
        mouse::{MouseMotion, MouseWheel},
    },
    prelude::*,
};
use selo::prelude::Workplane;

use crate::{
    pointer::PointerParams,
    state::AppState,
    workplane::{ActiveWorkplane, StoredWorkplane, WorkplaneParams},
};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>()
            .add_systems(Startup, setup_cameras)
            .add_systems(
                Update,
                (
                    move_camera.run_if(input_pressed(KeyCode::Space)),
                    rotate_camera.run_if(input_pressed(MouseButton::Middle)),
                    really_simple_zoom.run_if(not(input_pressed(KeyCode::ControlLeft))),
                )
                    .run_if(in_state(AppState::Algorithms)),
            )
            .add_systems(Update, align_camera_with_active_workplane);
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct MainCamera;

#[derive(SystemParam)]
pub struct CameraParams<'w, 's> {
    camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
}

impl CameraParams<'_, '_> {
    pub fn screen_ray_into_world(&self, screen_pos: Vec2) -> Option<Ray3d> {
        let (camera, global) = self.camera.single();
        camera.viewport_to_world(global, screen_pos)
    }

    pub fn screen_ray_onto_plane(&self, screen_pos: Vec2, workplane: Workplane) -> Option<Vec3> {
        self.screen_ray_into_world(screen_pos).and_then(|ray| {
            let dist = ray.intersect_plane(workplane.origin, workplane.plane)?;
            Some(ray.get_point(dist))
        })
    }
}

fn setup_cameras(mut cmds: Commands) {
    cmds.spawn((
        Name::new("Camera 3D"),
        MainCamera,
        Camera3dBundle {
            transform: Transform::from_translation(Vec3::Z * 10.0).looking_at(Vec3::ZERO, Vec3::Z),
            ..Default::default()
        },
    ));

    cmds.spawn((
        Name::new("Spotlight"),
        SpotLightBundle {
            transform: Transform::from_translation(Vec3::Z * 10.0).looking_at(Vec3::ZERO, Vec3::Z),
            spot_light: SpotLight {
                intensity: 5_000_000.0,
                ..Default::default()
            },
            ..Default::default()
        },
    ));
}

fn align_camera_with_active_workplane(
    workplane: Query<
        &StoredWorkplane,
        (
            With<ActiveWorkplane>,
            Or<(Changed<StoredWorkplane>, Added<ActiveWorkplane>)>,
        ),
    >,
    mut cam: Query<&mut Transform, With<MainCamera>>,
) {
    if let Ok(workplane) = workplane.get_single() {
        cam.iter_mut().for_each(|mut transform| {
            let up = transform.up();
            let normal = workplane.normal();
            let rotation = Quat::from_rotation_arc(up.as_vec3(), normal.as_vec3());
            *transform = transform
                .with_rotation(rotation)
                .looking_at(workplane.origin, normal);
        })
    }
}

fn move_camera(
    camera: CameraParams,
    pointer: PointerParams,
    mut mouse: EventReader<MouseMotion>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
    workplane: WorkplaneParams,
) {
    if let Some(pos) = pointer.screen_position() {
        let delta = mouse
            .read()
            .map(|drag| [pos, pos + drag.delta])
            .filter_map(|[start, end]| {
                Some([
                    camera.screen_ray_onto_plane(start, workplane.current())?,
                    camera.screen_ray_onto_plane(end, workplane.current())?,
                ])
            })
            .map(|[start, end]| end - start)
            .sum::<Vec3>();
        cam.iter_mut().for_each(|mut transform| {
            transform.translation -= delta;
        });
    }
}

fn rotate_camera(
    mut mouse: EventReader<MouseMotion>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
    workplane: WorkplaneParams,
) {
    let delta = mouse.read().map(|drag| drag.delta).sum::<Vec2>() * 0.0025;
    cam.iter_mut().for_each(|mut transform| {
        let x_rot = Quat::from_axis_angle(transform.local_x().as_vec3(), -delta.y);
        let z_rot = Quat::from_axis_angle(workplane.current().normal().as_vec3(), -delta.x);
        transform.rotate_around(workplane.current().origin(), x_rot * z_rot);
    });
}

fn really_simple_zoom(
    mut mouse: EventReader<MouseWheel>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
) {
    let delta = mouse.read().map(|scroll| scroll.y).sum::<f32>();
    cam.iter_mut().for_each(|mut transform| {
        let forward = transform.forward().as_vec3();
        transform.translation += delta * forward;
    });
}
