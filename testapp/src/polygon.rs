use bevy::{
    color::palettes,
    ecs::{entity::EntityHashSet, system::SystemParam},
    prelude::*,
};

use crate::{
    line::{Line, UnfinishedLine},
    point::Point,
    state::AppState,
};

pub struct PolygonPlugin;

impl Plugin for PolygonPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Polygon2D>()
            .add_systems(
                Update,
                (
                    crate::line::start_line.run_if(not(any_with_component::<UnfinishedLine>)),
                    crate::line::finish_line.run_if(any_with_component::<UnfinishedLine>),
                    finish_polygon,
                )
                    .run_if(in_state(AppState::Polygon)),
            )
            .add_systems(Update, render_polygons);
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Polygon2D {
    points: Vec<Entity>,
}

#[derive(SystemParam)]
pub struct PolygonParams<'w, 's> {
    polygon: Query<'w, 's, &'static Polygon2D>,
    lines: Query<'w, 's, &'static Line>,
    points: Query<'w, 's, &'static GlobalTransform, With<Point>>,
}

impl PolygonParams<'_, '_> {
    pub fn iter_polygons(&self) -> impl Iterator<Item = Vec<Vec2>> + '_ {
        self.polygon.iter().filter_map(|polygon| {
            polygon
                .points
                .iter()
                .map(|p| {
                    let line = self.lines.get(*p)?;
                    self.points
                        .get(line.start)
                        .map(|p| p.translation().truncate())
                })
                .collect::<Result<Vec<_>, _>>()
                .ok()
        })
    }
}

fn finish_polygon(
    mut cmds: Commands,
    lines: Query<(Entity, &Line)>,
    mut id: Local<usize>,
    polygons: Query<&Polygon2D>,
) {
    let mut visited = EntityHashSet::default();
    let mut circles = vec![];
    while let Some((entity, start_line)) =
        lines.iter().find(|(entity, _)| !visited.contains(entity))
    {
        visited.insert(entity);
        let mut stack = vec![vec![(entity, start_line)]];
        let mut finished = vec![];
        while let Some(work) = stack.pop() {
            finished.push(work.clone());
            if work.len() > 1
                && work.first().map(|(_, line)| line.start) == work.last().map(|(_, line)| line.end)
            {
                continue;
            }
            let (_, last_line) = work.last().unwrap();
            stack.extend(
                lines
                    .iter()
                    .filter(|(_, line)| last_line.end == line.start)
                    .map(|pair| {
                        visited.insert(pair.0);
                        let mut work_clone = work.clone();
                        work_clone.push(pair);
                        work_clone
                    }),
            );
        }

        circles.extend(
            finished
                .iter()
                .filter(|lines| lines.len() > 3)
                .filter(|lines| {
                    lines.first().map(|(_, line)| line.start)
                        == lines.last().map(|(_, line)| line.end)
                })
                .map(|lines| {
                    lines
                        .into_iter()
                        .map(|(entity, _)| *entity)
                        .collect::<Vec<_>>()
                }),
        )
    }

    circles
        .into_iter()
        .filter(|points| {
            !polygons
                .iter()
                .any(|polygon| polygon.points.iter().all(|point| points.contains(&point)))
        })
        .for_each(|points| {
            *id += 1;
            cmds.spawn((
                Name::new(format!("Polygon {n}", n = *id)),
                Polygon2D { points },
            ));
        });
}

fn render_polygons(mut gizmos: Gizmos, polygon: PolygonParams) {
    polygon.iter_polygons().for_each(|mut points| {
        if points.first() != points.last() {
            points.extend(points.first().cloned());
        }
        points
            .windows(2)
            .map(|win| (win[0], win[1]))
            .for_each(|(start, end)| {
                gizmos.line_2d(start, end, palettes::basic::AQUA);
            });
    });
}
