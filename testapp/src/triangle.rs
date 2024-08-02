use bevy::{
    color::palettes,
    ecs::{entity::EntityHashSet, system::SystemParam},
    prelude::*,
};
use bevy_mod_picking::prelude::*;

use crate::{
    line::{AttachedLines, Line, UnfinishedLine},
    point::Point,
    state::AppState,
};

pub struct TrianglePlugin;

impl Plugin for TrianglePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Triangle>()
            .add_systems(
                Update,
                (
                    start_line.run_if(not(any_with_component::<UnfinishedLine>)),
                    finish_line.run_if(any_with_component::<UnfinishedLine>),
                    finish_triangle,
                )
                    .run_if(in_state(AppState::Triangle)),
            )
            .add_systems(Update, render_triangles);
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Triangle {
    a: Entity,
    b: Entity,
    c: Entity,
}

#[derive(SystemParam)]
pub struct TriangleParams<'w, 's> {
    triangles: Query<'w, 's, &'static Triangle>,
    points: Query<'w, 's, &'static GlobalTransform, With<Point>>,
}

impl TriangleParams<'_, '_> {
    pub fn iter_triangles(&self) -> impl Iterator<Item = [Vec2; 3]> + '_ {
        self.triangles.iter().filter_map(|triangle| {
            self.points
                .get_many([triangle.a, triangle.b, triangle.c])
                .map(|poss| poss.map(|pos| pos.translation().truncate()))
                .ok()
        })
    }
}

fn start_line(
    mut cmds: Commands,
    points: Query<(Entity, &PickSelection), (With<Point>, Without<UnfinishedLine>)>,
) {
    if let Some((point, _)) = points.iter().filter(|(_, p)| p.is_selected).next() {
        cmds.entity(point).insert(UnfinishedLine);
    }
}

fn finish_line(
    mut cmds: Commands,
    mut points: Query<(Entity, &mut PickSelection), (With<Point>, Without<UnfinishedLine>)>,
    unfinished: Query<Entity, With<UnfinishedLine>>,
    mut attached_lines: Query<&mut AttachedLines>,
    mut id: Local<usize>,
) {
    let mut add_or_attach_line = |cmds: &mut Commands, point: Entity, line: Entity| {
        if let Ok(mut lines) = attached_lines.get_mut(point) {
            lines.insert(line);
        } else {
            cmds.entity(point)
                .insert(AttachedLines(EntityHashSet::from_iter(std::iter::once(
                    point,
                ))));
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
        add_or_attach_line(&mut cmds, start, line);
        add_or_attach_line(&mut cmds, end, line);
    }
}

fn finish_triangle(
    mut cmds: Commands,
    lines: Query<&Line>,
    mut id: Local<usize>,
    triangles: Query<&Triangle>,
) {
    if let Some([a, b, c]) = lines
        .iter()
        .enumerate()
        .flat_map(|(n, line_a)| {
            lines
                .iter()
                .enumerate()
                .skip(n)
                .map(move |(m, line_b)| (m, line_a, line_b))
        })
        .flat_map(|(m, line_a, line_b)| {
            lines
                .iter()
                .skip(m)
                .map(move |line_c| (line_a, line_b, line_c))
        })
        .filter_map(|(a, b, c)| {
            (a.end == b.start && b.end == c.start && c.end == a.start)
                .then_some([a.start, b.start, c.start])
        })
        .find(|[a, b, c]| {
            !triangles
                .iter()
                .any(|triangle| triangle.a == *a && triangle.b == *b && triangle.c == *c)
        })
    {
        *id += 1;
        cmds.spawn((
            Name::new(format!("Triangle {n}", n = *id)),
            Triangle { a, b, c },
        ));
    }
}

fn render_triangles(mut gizmos: Gizmos, triangles: TriangleParams) {
    triangles.iter_triangles().for_each(|[a, b, c]| {
        gizmos.primitive_2d(
            &Triangle2d::new(a, b, c),
            Vec2::ZERO,
            0.0,
            palettes::basic::TEAL,
        );
    })
}
