use bevy::{ecs::system::SystemParam, prelude::*};
use selo::prelude::Workplane;

use crate::camera::CameraParams;

#[derive(SystemParam)]
pub struct PointerParams<'w, 's> {
    window: Query<'w, 's, &'static Window>,
    camera: CameraParams<'w, 's>,
}

impl PointerParams<'_, '_> {
    pub fn screen_position(&self) -> Option<Vec2> {
        self.window.single().unwrap().cursor_position()
    }

    pub fn world_position_3d(&self, workplane: Workplane) -> Option<Vec3> {
        self.screen_position()
            .and_then(|screen_pos| self.camera.screen_ray_onto_plane(screen_pos, workplane))
    }
}
