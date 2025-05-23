use bevy::{
    color::palettes,
    ecs::{entity::EntityHashSet, query::QueryFilter, system::SystemParam},
    input::common_conditions::input_just_pressed,
    prelude::*,
};
use selo::prelude::Workplane;

use crate::{
    drop_system,
    point::{spawn_point, Point},
    pointer::PointerParams,
    state::AppState,
    workplane::{AttachedWorkplane, WorkplaneParams},
};

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<LinePoint>()
            .register_type::<JustLine>()
            .register_type::<UnfinishedLine>()
            .register_type::<Line>()
            .register_type::<AttachedLines>()
            .add_systems(
                Update,
                (
                    spawn_point
                        .pipe(line_point)
                        .pipe(line_start)
                        .pipe(drop_system)
                        .run_if(not(any_with_component::<UnfinishedLine>)),
                    spawn_point
                        .pipe(line_point)
                        .pipe(line_end)
                        .pipe(construct_lines)
                        .pipe(just_line)
                        .pipe(drop_system)
                        .run_if(any_with_component::<UnfinishedLine>),
                )
                    .run_if(in_state(AppState::Line).and(input_just_pressed(MouseButton::Left))),
            )
            .add_systems(
                Update,
                render_drawing_line
                    .run_if(in_state(AppState::Line).and(any_with_component::<UnfinishedLine>)),
            )
            .add_systems(Update, render_lines)
            .add_systems(OnExit(AppState::Line), cleanup_unfinished);
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct LinePoint;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct JustLine;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct UnfinishedLine;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Line {
    pub start: Entity,
    pub end: Entity,
}

#[derive(SystemParam)]
pub struct LineParams<'w, 's, F: QueryFilter + 'static = ()> {
    lines: Query<'w, 's, (&'static Line, &'static AttachedWorkplane), F>,
    points: Query<'w, 's, &'static GlobalTransform, With<Point>>,
}

impl<F: QueryFilter + 'static> LineParams<'_, '_, F> {
    pub fn iter_just_lines(&self) -> impl Iterator<Item = selo::Line<Vec3>> + '_ {
        self.iter_lines().map(|(line, _)| line)
    }

    pub fn iter_lines(&self) -> impl Iterator<Item = (selo::Line<Vec3>, Workplane)> + '_ {
        self.lines.iter().filter_map(|(line, wp)| {
            let line = selo::Line(
                self.points
                    .get_many([line.start, line.end])
                    .map(|poss| poss.map(|pos| pos.translation()))
                    .ok()?,
            );
            Some((line, **wp))
        })
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct AttachedLines {
    pub incomming: EntityHashSet,
    pub outgoing: EntityHashSet,
}

fn line_point(In(entity): In<Entity>, mut cmds: Commands) -> Entity {
    cmds.entity(entity).insert(LinePoint).id()
}

fn line_start(In(entity): In<Entity>, mut cmds: Commands) -> Entity {
    cmds.entity(entity).insert(UnfinishedLine).id()
}

fn line_end(
    In(entity): In<Entity>,
    mut cmds: Commands,
    unfinished: Query<Entity, With<UnfinishedLine>>,
) -> [(Entity, Entity); 1] {
    let start = unfinished.single();
    let end = entity;
    cmds.entity(start).remove::<UnfinishedLine>();
    [(start, end)]
}

pub fn construct_lines<const N: usize>(
    In(points): In<[(Entity, Entity); N]>,
    mut cmds: Commands,
    mut attached_lines: Query<&mut AttachedLines>,
    mut id: Local<usize>,
) -> [(Entity, (Entity, Entity)); N] {
    points.map(|(start, end)| {
        let line = line_constructor((start, end), &mut cmds, &mut attached_lines, &mut id);
        (line, (start, end))
    })
}

fn line_constructor(
    (start, end): (Entity, Entity),
    cmds: &mut Commands,
    attached_lines: &mut Query<&mut AttachedLines>,
    id: &mut usize,
) -> Entity {
    fn add_or_attach_line(
        cmds: &mut Commands,
        attached_lines: &mut Query<&mut AttachedLines, ()>,
        point: Entity,
        line: Entity,
        is_start: bool,
    ) {
        if let Ok(mut lines) = attached_lines.get_mut(point) {
            if is_start {
                lines.outgoing.insert(line);
            } else {
                lines.incomming.insert(line);
            }
        } else {
            let attached_lines = if is_start {
                AttachedLines {
                    outgoing: EntityHashSet::from_iter(std::iter::once(point)),
                    incomming: Default::default(),
                }
            } else {
                AttachedLines {
                    incomming: EntityHashSet::from_iter(std::iter::once(point)),
                    outgoing: Default::default(),
                }
            };
            cmds.entity(point).insert(attached_lines);
        }
    }

    *id += 1;
    let line = cmds
        .spawn((Name::new(format!("Line {n}", n = *id)), Line { start, end }))
        .id();
    add_or_attach_line(cmds, attached_lines, start, line, true);
    add_or_attach_line(cmds, attached_lines, end, line, false);
    line
}

fn just_line(In([(line, _)]): In<[(Entity, (Entity, Entity)); 1]>, mut cmds: Commands) -> Entity {
    cmds.entity(line).insert(JustLine).id()
}

pub fn render_drawing_line(
    mut gizmos: Gizmos,
    pointer: PointerParams,
    points: Query<&GlobalTransform, (With<Point>, With<UnfinishedLine>)>,
    workplane: WorkplaneParams,
) {
    let pointer_pos = pointer
        .world_position_3d(workplane.current())
        .unwrap_or_default();
    points
        .iter()
        .map(|transform| transform.translation())
        .for_each(|start| {
            gizmos.line(start, pointer_pos, palettes::basic::GREEN);
        });
}

fn render_lines(mut gizmos: Gizmos, lines: LineParams<With<JustLine>>) {
    let color = palettes::basic::GREEN;
    lines.iter_just_lines().for_each(|line| {
        let difference = line.dst() - line.src();
        gizmos.line(line.src(), line.dst(), color);
        gizmos.arrow(
            line.dst() - (difference.normalize() * 150.0).clamp_length_max(difference.length()),
            line.dst(),
            color,
        );
    })
}

fn cleanup_unfinished(mut cmds: Commands, unfinished: Query<Entity, With<UnfinishedLine>>) {
    unfinished.iter().for_each(|entity| {
        cmds.entity(entity).despawn_recursive();
    });
}
