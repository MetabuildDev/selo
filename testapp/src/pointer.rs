use bevy::{ecs::system::SystemParam, prelude::*};

#[derive(SystemParam)]
pub struct PointerParams<'w, 's> {
    window: Query<'w, 's, &'static Window>,
    camera: Query<'w, 's, (&'static Camera, &'static GlobalTransform)>,
}

impl PointerParams<'_, '_> {
    pub fn screen_position(&self) -> Option<Vec2> {
        self.window.single().cursor_position()
    }
    pub fn world_position(&self) -> Option<Vec2> {
        self.screen_position().and_then(|p| {
            let (camera, global) = self.camera.single();
            camera.viewport_to_world_2d(global, p)
        })
    }
}
