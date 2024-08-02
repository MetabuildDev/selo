use bevy::{
    color::palettes,
    ecs::{entity::EntityHashSet, system::SystemParam},
    prelude::*,
};
use bevy_mod_picking::prelude::*;

use crate::{point::Point, state::AppState};

pub struct LinePlugin;

impl Plugin for LinePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<UnfinishedLine>()
            .register_type::<Line>()
            .register_type::<AttachedLines>()
            .add_systems(
                Update,
                (
                    start_line.run_if(not(any_with_component::<UnfinishedLine>)),
                    finish_line.run_if(any_with_component::<UnfinishedLine>),
                )
                    .run_if(in_state(AppState::Line)),
            )
            .add_systems(Update, (render_lines,));
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct UnfinishedLine;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Component, Reflect)]
pub struct Line {
    pub start: Entity,
    pub end: Entity,
}

#[derive(SystemParam)]
pub struct LineParams<'w, 's> {
    lines: Query<'w, 's, &'static Line>,
    points: Query<'w, 's, &'static GlobalTransform, With<Point>>,
}

impl LineParams<'_, '_> {
    pub fn iter_lines(&self) -> impl Iterator<Item = [Vec2; 2]> + '_ {
        self.lines.iter().filter_map(|line| {
            self.points
                .get_many([line.start, line.end])
                .map(|poss| poss.map(|pos| pos.translation().truncate()))
                .ok()
        })
    }
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct AttachedLines {
    pub incomming: EntityHashSet,
    pub outgoing: EntityHashSet,
}

pub fn start_line(
    mut cmds: Commands,
    points: Query<(Entity, &PickSelection), (With<Point>, Without<UnfinishedLine>)>,
) {
    if let Some((point, _)) = points.iter().filter(|(_, p)| p.is_selected).next() {
        cmds.entity(point).insert(UnfinishedLine);
    }
}

pub fn finish_line(
    mut cmds: Commands,
    mut points: Query<(Entity, &mut PickSelection), (With<Point>, Without<UnfinishedLine>)>,
    unfinished: Query<Entity, With<UnfinishedLine>>,
    mut attached_lines: Query<&mut AttachedLines>,
    mut id: Local<usize>,
) {
    let mut add_or_attach_line =
        |cmds: &mut Commands, point: Entity, line: Entity, is_start: bool| {
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
        };
    if let Some((end, mut selection)) = points.iter_mut().filter(|(_, p)| p.is_selected).next() {
        *id += 1;
        selection.is_selected = false;
        let start = unfinished.single();
        cmds.entity(start).remove::<UnfinishedLine>();
        let line = cmds
            .spawn((Name::new(format!("Line {n}", n = *id)), Line { start, end }))
            .id();
        add_or_attach_line(&mut cmds, start, line, true);
        add_or_attach_line(&mut cmds, end, line, false);
    }
}

fn render_lines(mut gizmos: Gizmos, lines: LineParams) {
    let color = palettes::basic::GREEN;
    lines.iter_lines().for_each(|[start, end]| {
        let difference = end - start;
        gizmos.line_2d(start, end, color);
        gizmos.arrow_2d(
            end - (difference.normalize() * 150.0).clamp_length_max(difference.length()),
            end,
            color,
        );
    })
}
