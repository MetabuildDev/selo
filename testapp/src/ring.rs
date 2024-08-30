use bevy::{
    color::palettes, ecs::system::SystemParam, input::common_conditions::input_just_pressed,
    prelude::*,
};
use selo::prelude::Workplane;

use crate::{
    drop_system,
    line::construct_lines,
    point::{spawn_point, Point},
    pointer::PointerParams,
    state::AppState,
    workplane::{AttachedWorkplane, WorkplaneParams},
};

pub struct RingPlugin;

impl Plugin for RingPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Ring2D>()
            .register_type::<RingPoint>()
            .register_type::<RingLine>()
            .register_type::<LastRingPoint>()
            .register_type::<UnfinishedRingPoint>()
            .register_type::<UnfinishedRingLine>()
            .register_type::<RingPointIdSource>()
            .init_resource::<RingPointIdSource>()
            .add_systems(
                Update,
                (
                    (
                        spawn_point
                            .pipe(ring_start)
                            .pipe(ring_point)
                            .pipe(drop_system)
                            .run_if(not(any_with_component::<LastRingPoint>)),
                        spawn_point
                            .pipe(ring_point)
                            .pipe(ring_continue)
                            .pipe(construct_lines)
                            .pipe(unfinished_ring_line)
                            .pipe(drop_system)
                            .run_if(any_with_component::<LastRingPoint>),
                    )
                        .run_if(
                            in_state(AppState::Ring)
                                .and_then(input_just_pressed(MouseButton::Left)),
                        ),
                    (
                        get_sorted_ring_points
                            .pipe(get_last_line_point)
                            .pipe(construct_lines)
                            .pipe(ring_line)
                            .pipe(drop_system),
                        get_sorted_ring_points
                            .pipe(construct_ring)
                            .pipe(drop_system),
                        cleanup_construction_components,
                    )
                        .run_if(
                            in_state(AppState::Ring)
                                .and_then(input_just_pressed(MouseButton::Right))
                                .and_then(ring_finishable),
                        ),
                ),
            )
            .add_systems(
                Update,
                (
                    render_rings.run_if(any_with_component::<Ring2D>),
                    render_ring_construction.run_if(any_with_component::<RingPoint>),
                ),
            )
            .add_systems(OnExit(AppState::Ring), cleanup_unfinished);
    }
}

#[derive(Debug, Clone, Resource, Default, Reflect, Deref, DerefMut)]
pub struct RingPointIdSource(usize);

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct RingPoint(pub usize);

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct RingLine;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct LastRingPoint;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct UnfinishedRingPoint;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct UnfinishedRingLine;

#[derive(Debug, Clone, Component, Reflect)]
pub struct Ring2D {
    pub points: Vec<Entity>,
}

#[derive(SystemParam)]
pub struct RingParams<'w, 's> {
    ring: Query<'w, 's, (&'static Ring2D, &'static AttachedWorkplane)>,
    points: Query<'w, 's, (&'static GlobalTransform, &'static RingPoint), With<Point>>,
}

impl RingParams<'_, '_> {
    pub fn iter_just_rings(&self) -> impl Iterator<Item = selo::Ring<Vec3>> + '_ {
        self.iter_rings().map(|(ring, _)| ring)
    }

    pub fn iter_rings(&self) -> impl Iterator<Item = (selo::Ring<Vec3>, Workplane)> + '_ {
        self.ring.iter().filter_map(|(ring, wp)| {
            let points = selo::Ring::new(
                ring.points
                    .iter()
                    .map(|entity| {
                        self.points
                            .get(*entity)
                            .map(|(position, RingPoint(idx))| (idx, position.translation()))
                    })
                    .collect::<Result<Vec<_>, _>>()
                    .map(|mut vec| {
                        vec.sort_by_key(|(idx, _)| *idx);
                        vec.into_iter()
                            .map(|(_, position)| position)
                            .collect::<Vec<_>>()
                    })
                    .ok()?,
            );
            Some((points, **wp))
        })
    }
}

fn ring_finishable(points: Query<(), With<UnfinishedRingPoint>>) -> bool {
    points.iter().count() >= 3
}

fn ring_point(
    In(entity): In<Entity>,
    mut cmds: Commands,
    mut id: ResMut<RingPointIdSource>,
) -> Entity {
    **id += 1;
    cmds.entity(entity).insert(RingPoint(**id)).id()
}

fn ring_start(
    In(entity): In<Entity>,
    mut cmds: Commands,
    mut id_source: ResMut<RingPointIdSource>,
) -> Entity {
    **id_source = 0;
    cmds.entity(entity)
        .insert((LastRingPoint, UnfinishedRingPoint))
        .id()
}

fn ring_continue(
    In(entity): In<Entity>,
    mut cmds: Commands,
    last_point: Query<Entity, With<LastRingPoint>>,
) -> [(Entity, Entity); 1] {
    let start = cmds
        .entity(last_point.single())
        .remove::<LastRingPoint>()
        .id();
    let end = cmds
        .entity(entity)
        .insert((LastRingPoint, UnfinishedRingPoint))
        .id();
    [(start, end); 1]
}

fn ring_line(In(lines): In<[(Entity, (Entity, Entity)); 1]>, mut cmds: Commands) -> Entity {
    let [(entity, _)] = lines;
    cmds.entity(entity).insert(RingLine).id()
}

fn unfinished_ring_line(
    In(lines): In<[(Entity, (Entity, Entity)); 1]>,
    mut cmds: Commands,
) -> Entity {
    let [(entity, _)] = lines;
    cmds.entity(entity)
        .insert((RingLine, UnfinishedRingLine))
        .id()
}

fn get_sorted_ring_points(
    points: Query<(Entity, &RingPoint), With<UnfinishedRingPoint>>,
) -> Vec<Entity> {
    points
        .iter()
        .sort_by_key::<&RingPoint, _>(|RingPoint(id)| *id)
        .map(|(entity, _)| entity)
        .collect::<Vec<_>>()
}

fn get_last_line_point(In(points): In<Vec<Entity>>) -> [(Entity, Entity); 1] {
    let first = points.first().cloned().unwrap();
    let last = points.last().cloned().unwrap();

    [(last, first)]
}

fn construct_ring(In(points): In<Vec<Entity>>, mut cmds: Commands, mut id: Local<usize>) -> Entity {
    *id += 1;
    cmds.spawn((Name::new(format!("Ring {n}", n = *id)), Ring2D { points }))
        .id()
}

fn cleanup_construction_components(
    mut cmds: Commands,
    unfinished_line: Query<Entity, With<UnfinishedRingLine>>,
    unfinished_point: Query<Entity, With<UnfinishedRingPoint>>,
    last_point: Query<Entity, With<LastRingPoint>>,
) {
    unfinished_line.iter().for_each(|entity| {
        cmds.entity(entity).remove::<UnfinishedRingLine>();
    });
    unfinished_point.iter().for_each(|entity| {
        cmds.entity(entity).remove::<UnfinishedRingPoint>();
    });
    last_point.iter().for_each(|entity| {
        cmds.entity(entity).remove::<LastRingPoint>();
    });
}

fn render_rings(mut gizmos: Gizmos, ring: RingParams) {
    let colors = {
        use palettes::basic::*;
        [
            AQUA, BLUE, FUCHSIA, GREEN, LIME, MAROON, NAVY, OLIVE, PURPLE, SILVER, TEAL, YELLOW,
        ]
        .map(|c| c.mix(&WHITE, 0.5))
        // .windows(2)
        // .flat_map(|win| {
        //     let first = win[0];
        //     let second = win[1];
        //     (0..=2)
        //         .map(|n| n as f32 / 2.0)
        //         .map(move |percent| first.mix(&second, percent))
        // })
        // .collect::<Vec<_>>()
    };
    ring.iter_just_rings()
        .zip(colors.into_iter().cycle())
        .for_each(|(ring, color)| {
            ring.lines().for_each(|line| {
                gizmos.line(line.src(), line.dst(), color);
            });
        });
}

fn render_ring_construction(
    mut gizmos: Gizmos,
    points: Query<(&GlobalTransform, &RingPoint), With<UnfinishedRingPoint>>,
    pointer: PointerParams,
    workplane: WorkplaneParams,
) {
    let points = points
        .iter()
        .sort_by_key::<&RingPoint, _>(|RingPoint(id)| *id)
        .map(|(transform, _)| transform.translation())
        .collect::<Vec<_>>();

    points
        .windows(2)
        .map(|win| (win[0], win[1]))
        .for_each(|(start, end)| {
            gizmos.line(start, end, palettes::basic::AQUA);
        });

    let pointer_pos = pointer
        .world_position_3d(workplane.current())
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
    unfinished: Query<Entity, Or<(With<UnfinishedRingLine>, With<UnfinishedRingPoint>)>>,
) {
    unfinished.iter().for_each(|entity| {
        cmds.entity(entity).despawn_recursive();
    });
}
