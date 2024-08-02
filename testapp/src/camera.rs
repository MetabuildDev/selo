use bevy::prelude::*;
use math::Mirror2D;

use crate::{pointer::PointerParams, state::AppState};

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, |mut cmds: Commands| {
            cmds.spawn(Camera2dBundle::default());
        })
        .add_systems(OnEnter(AppState::Move), start_camera_move)
        .add_systems(OnExit(AppState::Move), stop_camera_move)
        .add_systems(
            Update,
            camera_move.run_if(any_with_component::<PointerMoveStart>),
        );
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
struct PointerMoveStart {
    pointer: Vec2,
    camera: Vec3,
}

fn start_camera_move(
    mut cmds: Commands,
    pointer: PointerParams,
    camera: Query<(Entity, &Transform), With<Camera>>,
) {
    let (camera_entity, camera_transform) = camera.single();
    cmds.entity(camera_entity).insert(PointerMoveStart {
        pointer: pointer.screen_position().unwrap_or_default(),
        camera: camera_transform.translation,
    });
}

fn camera_move(mut camera: Query<(&mut Transform, &PointerMoveStart)>, pointer: PointerParams) {
    let (mut transform, move_start) = camera.single_mut();
    if let Some(current_pointer_pos) = pointer.screen_position() {
        let delta = current_pointer_pos - move_start.pointer;
        transform.translation = move_start.camera + delta.mirror_x().extend(0.0);
    }
}

fn stop_camera_move(mut cmds: Commands, stopping: Query<Entity, With<PointerMoveStart>>) {
    stopping.iter().for_each(|entity| {
        cmds.entity(entity).remove::<PointerMoveStart>();
    });
}
