use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use bevy_mod_picking::prelude::*;

use crate::{
    camera::CameraParams, drop_system, pointer::PointerParams, state::AppState,
    working_plane::WorkplaneParams,
};

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Point>()
            .register_type::<JustPoint>()
            .register_type::<DraggedPosition>()
            .add_systems(
                Update,
                spawn_point.pipe(just_point).pipe(drop_system).run_if(
                    in_state(AppState::Point).and_then(input_just_pressed(MouseButton::Left)),
                ),
            )
            .add_systems(
                OnEnter(AppState::Algorithms),
                (insert_drag_observers, insert_pickability),
            )
            .add_systems(
                OnExit(AppState::Algorithms),
                (remove_drag_observers, remove_pickability),
            )
            .add_systems(OnEnter(AppState::Triangle), insert_pickability)
            .add_systems(OnExit(AppState::Triangle), remove_pickability)
            .add_systems(OnEnter(AppState::Ring), insert_pickability)
            .add_systems(OnExit(AppState::Ring), remove_pickability)
            .add_systems(
                Update,
                apply_dragged_position.run_if(any_with_component::<DraggedPosition>),
            );
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct Point;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct JustPoint;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct DraggedPosition {
    position: Vec2,
}

pub fn spawn_point(
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pointer: PointerParams,
    mut id: Local<usize>,
    working_plane: WorkplaneParams,
) -> Entity {
    *id += 1;

    let name = Name::new(format!("Point {n}", n = *id));
    let position = pointer
        .world_position_3d(working_plane.current())
        .unwrap_or_default();

    let mesh = meshes.add(Circle::new(0.025));
    let material = materials.add(StandardMaterial::from_color(Color::from(
        palettes::basic::WHITE,
    )));

    cmds.spawn((
        Point,
        name,
        MaterialMeshBundle {
            mesh,
            material,
            transform: Transform::from_translation(position),
            ..Default::default()
        },
    ))
    .id()
}

fn just_point(In(entity): In<Entity>, mut cmds: Commands) -> Entity {
    cmds.entity(entity).insert(JustPoint).id()
}

fn insert_drag_observers(mut cmds: Commands, points: Query<Entity, With<Point>>) {
    points.iter().for_each(|point| {
        cmds.entity(point)
            .insert(On::<Pointer<Drag>>::commands_mut(|drag, cmds| {
                let position = drag.pointer_location.position;
                let rounded_position = (position / 5.0).round() * 5.0;
                cmds.entity(drag.target).insert(DraggedPosition {
                    position: rounded_position,
                });
            }));
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

fn apply_dragged_position(
    mut cmds: Commands,
    mut dragged: Query<(Entity, &mut Transform, &DraggedPosition)>,
    camera: CameraParams,
    working_plane: WorkplaneParams,
) {
    dragged
        .iter_mut()
        .filter_map(|(entity, transform, dragged)| {
            let pos = camera.screen_ray_onto_plane(dragged.position, working_plane.current())?;
            Some((entity, transform, pos))
        })
        .for_each(|(entity, mut transform, pos3d)| {
            cmds.entity(entity).remove::<DraggedPosition>();
            transform.translation = pos3d;
        });
}
