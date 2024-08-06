use bevy::{ecs::system::SystemParam, prelude::*};

use crate::camera::CameraParams;

#[derive(SystemParam)]
pub struct PointerParams<'w, 's> {
    window: Query<'w, 's, &'static Window>,
    camera: CameraParams<'w, 's>,
}

impl PointerParams<'_, '_> {
    pub fn screen_position(&self) -> Option<Vec2> {
        self.window.single().cursor_position()
    }

    pub fn world_position_3d(&self) -> Option<Vec3> {
        self.screen_position()
            .and_then(|screen_pos| self.camera.screen_ray_into_world(screen_pos))
            .and_then(|ray| {
                let dist = ray.intersect_plane(Vec3::ZERO, InfinitePlane3d { normal: Dir3::Z })?;
                Some(ray.get_point(dist))
            })
    }
}
