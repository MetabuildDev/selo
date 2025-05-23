use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};

use crate::{
    camera::CameraParams, drop_system, pointer::PointerParams, state::AppState,
    workplane::WorkplaneParams,
};

pub struct PointPlugin;

impl Plugin for PointPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Point>()
            .register_type::<JustPoint>()
            .register_type::<DraggedPosition>()
            .add_systems(
                Update,
                spawn_point
                    .pipe(just_point)
                    .pipe(drop_system)
                    .run_if(in_state(AppState::Point).and(input_just_pressed(MouseButton::Left))),
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
    workplane: WorkplaneParams,
) -> Entity {
    *id += 1;

    let name = Name::new(format!("Point {n}", n = *id));
    let position = pointer
        .world_position_3d(workplane.current())
        .unwrap_or_default();

    let mesh = meshes.add(Circle::new(0.005));
    let material = materials.add(StandardMaterial::from_color(Color::from(
        palettes::basic::WHITE,
    )));

    cmds.spawn((
        Point,
        name,
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
    ))
    .id()
}

fn just_point(In(entity): In<Entity>, mut cmds: Commands) -> Entity {
    cmds.entity(entity).insert(JustPoint).id()
}

fn insert_drag_observers(mut cmds: Commands, points: Query<Entity, With<Point>>) {
    points.iter().for_each(|point| {
        cmds.spawn((
            Observer::new(|drag: Trigger<Pointer<Drag>>, mut cmds: Commands| {
                let position = drag.pointer_location.position;
                let rounded_position = (position / 5.0).round() * 5.0;
                cmds.entity(drag.target).insert(DraggedPosition {
                    position: rounded_position,
                });
            })
            .with_entity(point),
            DragPointObserver,
        ));
    });
}

#[derive(Component)]
pub struct DragPointObserver;

fn remove_drag_observers(mut cmds: Commands, points: Query<Entity, With<DragPointObserver>>) {
    points.iter().for_each(|point| {
        cmds.entity(point).remove::<DragPointObserver>();
    });
}

fn insert_pickability(mut cmds: Commands, points: Query<Entity, With<Point>>) {
    points.iter().for_each(|point| {
        cmds.entity(point).insert(PickingBehavior::default());
    });
}

fn remove_pickability(mut cmds: Commands, points: Query<Entity, With<Point>>) {
    points.iter().for_each(|point| {
        cmds.entity(point).insert(PickingBehavior::IGNORE);
    });
}

fn apply_dragged_position(
    mut cmds: Commands,
    mut dragged: Query<(Entity, &mut Transform, &DraggedPosition)>,
    camera: CameraParams,
    workplane: WorkplaneParams,
) {
    dragged
        .iter_mut()
        .filter_map(|(entity, transform, dragged)| {
            let pos = camera.screen_ray_onto_plane(dragged.position, workplane.current())?;
            Some((entity, transform, pos))
        })
        .for_each(|(entity, mut transform, pos3d)| {
            cmds.entity(entity).remove::<DraggedPosition>();
            transform.translation = pos3d;
        });
}
