use bevy::{ecs::system::SystemParam, prelude::*};

use crate::camera::MainCamera;

#[derive(SystemParam)]
pub struct PointerParams<'w, 's> {
    window: Query<'w, 's, &'static Window>,
    camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform), With<MainCamera>>,
}

impl PointerParams<'_, '_> {
    pub fn screen_position(&self) -> Option<Vec2> {
        self.window.single().cursor_position()
    }

    pub fn world_position_2d(&self) -> Option<Vec2> {
        self.screen_position().and_then(|p| {
            let (camera, global) = self.camera.single();
            camera.viewport_to_world_2d(global, p)
        })
    }

    pub fn world_position_3d(&self) -> Option<Vec3> {
        self.screen_position()
            .and_then(|p| {
                let (camera, global) = self.camera.single();
                camera.viewport_to_world(global, p)
            })
            .and_then(|ray| {
                let dist = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d { normal: Dir3::Z })?;
                Some(ray.get_point(dist))
            })
    }
}
