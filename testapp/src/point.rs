use bevy::{
    color::palettes, input::common_conditions::input_just_pressed, prelude::*,
    sprite::MaterialMesh2dBundle,
};
use bevy_mod_picking::prelude::*;
use math::Mirror2D;

use crate::{pointer::PointerParams, state::AppState};

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Point>()
            .add_systems(
                Update,
                (
                    spawn_point.run_if(in_state(AppState::Point)),
                    (crate::state::unselect_everything, spawn_picked_point)
                        .chain()
                        .run_if(in_state(AppState::Line)),
                )
                    .run_if(input_just_pressed(MouseButton::Right)),
            )
            .add_systems(
                OnEnter(AppState::Algorithms),
                (insert_drag_observers, insert_pickability),
            )
            .add_systems(
                OnExit(AppState::Algorithms),
                (remove_drag_observers, remove_pickability),
            )
            .add_systems(OnEnter(AppState::Line), insert_pickability)
            .add_systems(OnExit(AppState::Line), remove_pickability)
            .add_systems(OnEnter(AppState::Triangle), insert_pickability)
            .add_systems(OnExit(AppState::Triangle), remove_pickability)
            .add_systems(OnEnter(AppState::Polygon), insert_pickability)
            .add_systems(OnExit(AppState::Polygon), remove_pickability);
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct Point;

fn spawn_point(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pointer: PointerParams,
    mut id: Local<usize>,
) {
    *id += 1;

    let name = Name::new(format!("Point {n}", n = *id));
    let position = pointer.world_position().unwrap_or_default();

    let mesh = meshes.add(Circle::new(10.0)).into();
    let material = materials.add(ColorMaterial::from(Color::from(palettes::basic::WHITE)));

    cmds.spawn((
        Point,
        name,
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        },
    ));
}

fn spawn_picked_point(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    pointer: PointerParams,
    mut id: Local<usize>,
) {
    *id += 1;

    let name = Name::new(format!("Point {n}", n = *id));
    let position = pointer.world_position().unwrap_or_default();

    let mesh = meshes.add(Circle::new(10.0)).into();
    let material = materials.add(ColorMaterial::from(Color::from(palettes::basic::WHITE)));

    cmds.spawn((
        Point,
        name,
        MaterialMesh2dBundle {
            mesh,
            material,
            transform: Transform::from_translation(position.extend(0.0)),
            ..Default::default()
        },
        PickableBundle {
            selection: PickSelection { is_selected: true },
            ..Default::default()
        },
    ));
}

fn insert_drag_observers(mut cmds: Commands, points: Query<Entity, With<Point>>) {
    points.iter().for_each(|point| {
        cmds.entity(point)
            .insert(On::<Pointer<Drag>>::target_component_mut::<Transform>(
                |drag, transform| {
                    transform.translation += drag.delta.mirror_y().extend(0.0);
                },
            ));
    });
}

fn remove_drag_observers(mut cmds: Commands, points: Query<Entity, With<Point>>) {
    points.iter().for_each(|point| {
        cmds.entity(point).remove::<On<Pointer<Drag>>>();
    });
}

fn insert_pickability(mut cmds: Commands, points: Query<Entity, With<Point>>) {
    points.iter().for_each(|point| {
        cmds.entity(point).insert(PickableBundle::default());
    });
}

fn remove_pickability(mut cmds: Commands, points: Query<Entity, With<Point>>) {
    points.iter().for_each(|point| {
        cmds.entity(point).remove::<PickableBundle>();
    });
}
