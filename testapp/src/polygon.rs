use bevy::{
    color::palettes, ecs::system::SystemParam, input::common_conditions::input_just_pressed,
    prelude::*,
};
use math::prelude::WorkingPlane;

use crate::{
    drop_system,
    line::construct_lines,
    point::{spawn_point, Point},
    pointer::PointerParams,
    state::AppState,
    working_plane::{AttachedWorkingPlane, WorkingPlaneParams},
};

pub struct PolygonPlugin;

impl Plugin for PolygonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Polygon2D>()
            .register_type::<PolygonPoint>()
            .register_type::<PolygonLine>()
            .register_type::<LastPolyPoint>()
            .register_type::<UnfinishedPolyPoint>()
            .register_type::<UnfinishedPolyLine>()
            .register_type::<PolygonPointIdSource>()
            .init_resource::<PolygonPointIdSource>()
            .add_systems(
                Update,
                (
                    (
                        spawn_point
                            .pipe(polygon_start)
                            .pipe(polygon_point)
                            .pipe(drop_system)
                            .run_if(not(any_with_component::<LastPolyPoint>)),
                        spawn_point
                            .pipe(polygon_point)
                            .pipe(polygon_continue)
                            .pipe(construct_lines)
                            .pipe(unfinished_polygon_line)
                            .pipe(drop_system)
                            .run_if(any_with_component::<LastPolyPoint>),
                    )
                        .run_if(
                            in_state(AppState::Polygon)
                                .and_then(input_just_pressed(MouseButton::Left)),
                        ),
                    (
                        get_sorted_polygon_points
                            .pipe(get_last_line_point)
                            .pipe(construct_lines)
                            .pipe(polygon_line)
                            .pipe(drop_system),
                        get_sorted_polygon_points
                            .pipe(construct_polygon)
                            .pipe(drop_system),
                        cleanup_construction_components,
                    )
                        .run_if(
                            in_state(AppState::Polygon)
                                .and_then(input_just_pressed(MouseButton::Right))
                                .and_then(polygon_finishable),
                        ),
                ),
            )
            .add_systems(
                Update,
                (
                    render_polygons.run_if(any_with_component::<Polygon2D>),
                    render_polygon_construction.run_if(any_with_component::<PolygonPoint>),
                ),
            )
            .add_systems(OnExit(AppState::Polygon), cleanup_unfinished);
    }
}

#[derive(Debug, Clone, Resource, Default, Reflect, Deref, DerefMut)]
pub struct PolygonPointIdSource(usize);

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct PolygonPoint(usize);

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct PolygonLine;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct LastPolyPoint;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct UnfinishedPolyPoint;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct UnfinishedPolyLine;

#[derive(Debug, Clone, Component, Reflect)]
pub struct Polygon2D {
    points: Vec<Entity>,
}

#[derive(SystemParam)]
pub struct PolygonParams<'w, 's> {
    polygon: Query<'w, 's, (&'static Polygon2D, &'static AttachedWorkingPlane)>,
    points: Query<'w, 's, (&'static GlobalTransform, &'static PolygonPoint), With<Point>>,
}

impl PolygonParams<'_, '_> {
    pub fn iter_just_polygons(&self) -> impl Iterator<Item = Vec<Vec3>> + '_ {
        self.iter_polygons().map(|(polygon, _)| polygon)
    }
    pub fn iter_polygons(&self) -> impl Iterator<Item = (Vec<Vec3>, WorkingPlane)> + '_ {
        self.polygon.iter().filter_map(|(polygon, wp)| {
            let points = polygon
                .points
                .iter()
                .map(|entity| {
                    self.points
                        .get(*entity)
                        .map(|(position, PolygonPoint(idx))| (idx, position.translation()))
                })
                .collect::<Result<Vec<_>, _>>()
                .map(|mut vec| {
                    vec.sort_by_key(|(idx, _)| *idx);
                    vec.into_iter()
                        .map(|(_, position)| position)
                        .collect::<Vec<_>>()
                })
                .ok()?;
            Some((points, **wp))
        })
    }
}

fn polygon_finishable(points: Query<(), With<PolygonPoint>>) -> bool {
    points.iter().count() >= 3
}

fn polygon_point(
    In(entity): In<Entity>,
    mut cmds: Commands,
    mut id: ResMut<PolygonPointIdSource>,
) -> Entity {
    **id += 1;
    cmds.entity(entity).insert(PolygonPoint(**id)).id()
}

fn polygon_start(
    In(entity): In<Entity>,
    mut cmds: Commands,
    mut id_source: ResMut<PolygonPointIdSource>,
) -> Entity {
    **id_source = 0;
    cmds.entity(entity)
        .insert((LastPolyPoint, UnfinishedPolyPoint))
        .id()
}

fn polygon_continue(
    In(entity): In<Entity>,
    mut cmds: Commands,
    last_point: Query<Entity, With<LastPolyPoint>>,
) -> [(Entity, Entity); 1] {
    let start = cmds
        .entity(last_point.single())
        .remove::<LastPolyPoint>()
        .id();
    let end = cmds
        .entity(entity)
        .insert((LastPolyPoint, UnfinishedPolyPoint))
        .id();
    [(start, end); 1]
}

fn polygon_line(In(lines): In<[(Entity, (Entity, Entity)); 1]>, mut cmds: Commands) -> Entity {
    let [(entity, _)] = lines;
    cmds.entity(entity).insert(PolygonLine).id()
}

fn unfinished_polygon_line(
    In(lines): In<[(Entity, (Entity, Entity)); 1]>,
    mut cmds: Commands,
) -> Entity {
    let [(entity, _)] = lines;
    cmds.entity(entity)
        .insert((PolygonLine, UnfinishedPolyLine))
        .id()
}

fn get_sorted_polygon_points(
    points: Query<(Entity, &PolygonPoint), With<UnfinishedPolyPoint>>,
) -> Vec<Entity> {
    points
        .iter()
        .sort_by_key::<&PolygonPoint, _>(|PolygonPoint(id)| *id)
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>()
}

fn get_last_line_point(In(points): In<Vec<Entity>>) -> [(Entity, Entity); 1] {
    let first = points.first().cloned().unwrap();
    let last = points.last().cloned().unwrap();

    [(last, first)]
}

fn construct_polygon(
    In(points): In<Vec<Entity>>,
    mut cmds: Commands,
    mut id: Local<usize>,
) -> Entity {
    *id += 1;
    cmds.spawn((
        Name::new(format!("Polygon {n}", n = *id)),
        Polygon2D { points },
    ))
    .id()
}

fn cleanup_construction_components(
    mut cmds: Commands,
    unfinished_line: Query<Entity, With<UnfinishedPolyLine>>,
    unfinished_point: Query<Entity, With<UnfinishedPolyPoint>>,
    last_point: Query<Entity, With<LastPolyPoint>>,
) {
    unfinished_line.iter().for_each(|entity| {
        cmds.entity(entity).remove::<UnfinishedPolyLine>();
    });
    unfinished_point.iter().for_each(|entity| {
        cmds.entity(entity).remove::<UnfinishedPolyPoint>();
    });
    last_point.iter().for_each(|entity| {
        cmds.entity(entity).remove::<LastPolyPoint>();
    });
}

fn render_polygons(mut gizmos: Gizmos, polygon: PolygonParams) {
    polygon.iter_just_polygons().for_each(|mut points| {
        if points.first() != points.last() {
            points.extend(points.first().cloned());
        }
        points
            .windows(2)
            .map(|win| (win[0], win[1]))
            .for_each(|(start, end)| {
                gizmos.line(start, end, palettes::basic::AQUA);
            });
    });
}

fn render_polygon_construction(
    mut gizmos: Gizmos,
    points: Query<(&GlobalTransform, &PolygonPoint), With<UnfinishedPolyPoint>>,
    pointer: PointerParams,
    working_plane: WorkingPlaneParams,
) {
    let points = points
        .iter()
        .sort_by_key::<&PolygonPoint, _>(|PolygonPoint(id)| *id)
        .map(|(transform, _)| transform.translation())
        .collect::<Vec<_>>();

    points
        .windows(2)
        .map(|win| (win[0], win[1]))
        .for_each(|(start, end)| {
            gizmos.line(start, end, palettes::basic::AQUA);
        });

    let pointer_pos = pointer
        .world_position_3d(working_plane.current())
        .unwrap_or_default();
    if let Some(end) = points.last().cloned() {
        gizmos.line(pointer_pos, end, palettes::basic::AQUA);
    }
    if let Some(end) = points.first().cloned() {
        gizmos.line(pointer_pos, end, palettes::basic::AQUA);
    }
}

fn cleanup_unfinished(
    mut cmds: Commands,
    unfinished: Query<Entity, Or<(With<UnfinishedPolyLine>, With<UnfinishedPolyPoint>)>>,
) {
    unfinished.iter().for_each(|entity| {
        cmds.entity(entity).despawn_recursive();
    });
}
