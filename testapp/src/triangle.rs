use bevy::{color::palettes, ecs::system::SystemParam, prelude::*};

use crate::{
    line::{Line, UnfinishedLine},
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
                    crate::line::start_line.run_if(not(any_with_component::<UnfinishedLine>)),
                    crate::line::finish_line.run_if(any_with_component::<UnfinishedLine>),
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
            !triangles.iter().any(|triangle| {
                [triangle.a, triangle.b, triangle.c]
                    .iter()
                    .all(|point| [a, b, c].contains(&point))
            })
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
