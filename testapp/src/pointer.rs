use bevy::{ecs::system::SystemParam, prelude::*};
use math::prelude::WorkingPlane;

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

    pub fn world_position_3d(&self, working_plane: WorkingPlane) -> Option<Vec3> {
        self.screen_position()
            .and_then(|screen_pos| self.camera.screen_ray_onto_plane(screen_pos, working_plane))
    }
}
