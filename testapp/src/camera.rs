use bevy::{
    ecs::system::SystemParam,
    input::{
        common_conditions::input_pressed,
        mouse::{MouseMotion, MouseWheel},
    },
    prelude::*,
    render::camera::ViewportConversionError,
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
                    zoom_camera.run_if(not(input_pressed(KeyCode::ControlLeft))),
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
    pub fn screen_ray_into_world(
        &self,
        screen_pos: Vec2,
    ) -> Result<Ray3d, ViewportConversionError> {
        let (camera, global) = self.camera.single().unwrap();
        camera.viewport_to_world(global, screen_pos)
    }

    pub fn screen_ray_onto_plane(&self, screen_pos: Vec2, workplane: Workplane) -> Option<Vec3> {
        self.screen_ray_into_world(screen_pos).ok().and_then(|ray| {
            let dist = ray.intersect_plane(workplane.origin, workplane.plane)?;
            Some(ray.get_point(dist))
        })
    }

    // pub fn world_into_screen
}

fn setup_cameras(mut cmds: Commands) {
    cmds.spawn((
        Name::new("Camera 3D"),
        MainCamera,
        Camera3d::default(),
        Projection::Perspective(PerspectiveProjection {
            near: 0.01,
            ..default()
        }),
        Transform::from_translation(Vec3::Z * 10.0).looking_at(Vec3::ZERO, Vec3::Z),
    ));

    cmds.spawn((
        Name::new("Spotlight"),
        SpotLight {
            intensity: 5_000_000.0,
            ..default()
        },
        Transform::from_translation(Vec3::Z * 10.0).looking_at(Vec3::ZERO, Vec3::Z),
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
    if let Ok(workplane) = workplane.single() {
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
    mut cam: Query<&mut Transform, With<MainCamera>>,
    workplane: WorkplaneParams,
    keys: Res<ButtonInput<KeyCode>>,
    // Anchor point that will be maintained on the cursor throughout the panning action
    mut anchor: Local<Option<Vec3>>,
) {
    if keys.just_pressed(KeyCode::Space) {
        *anchor = pointer
            .screen_position()
            .and_then(|pos| camera.screen_ray_onto_plane(pos, workplane.current()));
    } else {
        let Some(anchor) = *anchor else {
            return;
        };
        let Some(screen_pos) = pointer.screen_position() else {
            return;
        };
        let Some(current_target) = camera.screen_ray_onto_plane(screen_pos, workplane.current())
        else {
            return;
        };
        let mut camera_tr = cam.single_mut().unwrap();
        let to_camera = camera_tr.translation - current_target;
        camera_tr.translation = anchor + to_camera;
    }
}

fn rotate_camera(
    camera: CameraParams,
    pointer: PointerParams,
    mut mouse: EventReader<MouseMotion>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
    workplane: WorkplaneParams,
    buttons: Res<ButtonInput<MouseButton>>,
    mut pivot: Local<Option<Vec3>>,
) {
    if buttons.just_pressed(MouseButton::Middle) {
        *pivot = pointer
            .screen_position()
            .and_then(|pos| camera.screen_ray_onto_plane(pos, workplane.current()));
    } else {
        let Some(pivot) = *pivot else {
            return;
        };
        let delta = mouse.read().map(|drag| drag.delta).sum::<Vec2>() * 0.0025;
        cam.iter_mut().for_each(|mut transform| {
            let x_rot = Quat::from_axis_angle(transform.local_x().as_vec3(), -delta.y);
            let z_rot = Quat::from_axis_angle(workplane.current().normal().as_vec3(), -delta.x);
            transform.rotate_around(pivot, x_rot * z_rot);
        });
    }
}

fn zoom_camera(
    camera: CameraParams,
    pointer: PointerParams,
    workplane: WorkplaneParams,
    mut mouse: EventReader<MouseWheel>,
    mut cam: Query<&mut Transform, With<MainCamera>>,
) {
    let Some(center) = pointer
        .screen_position()
        .and_then(|pos| camera.screen_ray_onto_plane(pos, workplane.current()))
    else {
        return;
    };
    let delta = mouse.read().map(|scroll| scroll.y).sum::<f32>();
    cam.iter_mut().for_each(|mut transform| {
        let to_camera = transform.translation - center;
        let scaling = 2f32.powf(-delta * 0.25);
        if to_camera.length() * scaling < 0.02 {
            return;
        }
        transform.translation = center + to_camera * scaling;
    });
}
