use bevy::{ecs::system::SystemParam, prelude::*};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>()
            .add_systems(Startup, setup_cameras);
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

    pub fn screen_ray_onto_xy(&self, screen_pos: Vec2) -> Option<Vec3> {
        self.screen_ray_into_world(screen_pos).and_then(|ray| {
            let dist = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d { normal: Dir3::Z })?;
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
