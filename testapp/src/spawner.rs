use bevy::{color::palettes, ecs::entity::EntityHashSet, prelude::*};
use selo::IterPoints;

use crate::{
    line::{AttachedLines, Line},
    point::Point,
    ring::{Ring2D, RingLine, RingPoint},
    triangle::{Triangle, TriangleLine, TrianglePoint},
};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTriangle>()
            .add_event::<SpawnRing>()
            .add_systems(
                Update,
                (
                    spawn_triangle.run_if(on_event::<SpawnTriangle>()),
                    spawn_ring.run_if(on_event::<SpawnRing>()),
                ),
            );
    }
}

#[derive(Debug, Clone, Event)]
pub struct SpawnTriangle(pub selo::Triangle<Vec3>);

fn spawn_triangle(
    mut spawn_events: EventReader<SpawnTriangle>,

    mut cmds: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut ids: Local<(usize, usize, usize)>,

    mut attached_lines: Query<&mut AttachedLines, ()>,
) {
    spawn_events
        .read()
        .for_each(|SpawnTriangle(selo::Triangle([a, b, c]))| {
            let [a, b, c] = [a, b, c].map(|position| {
                spawn_point_inner(
                    *position,
                    &mut cmds,
                    &mut meshes,
                    &mut materials,
                    &mut ids.0,
                    |_id| TrianglePoint,
                )
            });
            let [_ab, _bc, _ca] = [(a, b), (b, c), (c, a)].map(|(start, end)| {
                spawn_line_inner(
                    start,
                    end,
                    &mut cmds,
                    &mut attached_lines,
                    &mut ids.2,
                    |_| TriangleLine,
                )
            });

            ids.2 += 1;

            cmds.spawn((
                Name::new(format!("Triangle {n}", n = ids.2)),
                Triangle { a, b, c },
            ));
        });
}

#[derive(Debug, Clone, Event)]
pub struct SpawnRing(pub selo::Ring<Vec3>);

fn spawn_ring(
    mut spawn_events: EventReader<SpawnRing>,

    mut cmds: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut ids: Local<(usize, usize, usize)>,

    mut attached_lines: Query<&mut AttachedLines, ()>,
) {
    spawn_events.read().for_each(|SpawnRing(ring)| {
        let points = ring
            .iter_points()
            .map(|position| {
                spawn_point_inner(
                    position,
                    &mut cmds,
                    &mut meshes,
                    &mut materials,
                    &mut ids.0,
                    |id| RingPoint(id),
                )
            })
            .collect::<Vec<_>>();
        let mut lines = points.clone();
        lines.extend(lines.first().cloned());
        let _lines = lines
            .windows(2)
            .map(|win| [win[0], win[1]])
            .map(|[start, end]| {
                spawn_line_inner(
                    start,
                    end,
                    &mut cmds,
                    &mut attached_lines,
                    &mut ids.2,
                    |_| RingLine,
                )
            })
            .collect::<Vec<_>>();

        ids.2 += 1;

        cmds.spawn((Name::new(format!("Ring {n}", n = ids.2)), Ring2D { points }));
    });
}

fn spawn_point_inner<C: Component>(
    position: Vec3,

    cmds: &mut Commands,

    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,

    id: &mut usize,

    extra_component: impl Fn(usize) -> C,
) -> Entity {
    *id += 1;

    let name = Name::new(format!("Point {n}", n = *id));

    let mesh = meshes.add(Circle::new(0.025));
    let material = materials.add(StandardMaterial::from_color(Color::from(
        palettes::basic::WHITE,
    )));

    cmds.spawn((
        Point,
        extra_component(*id),
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

fn spawn_line_inner<C: Component>(
    start: Entity,
    end: Entity,

    cmds: &mut Commands,

    attached_lines: &mut Query<&mut AttachedLines, ()>,

    id: &mut usize,

    extra_component: impl Fn(usize) -> C,
) -> Entity {
    *id += 1;
    let line = cmds
        .spawn((
            Name::new(format!("Line {n}", n = *id)),
            extra_component(*id),
            Line { start, end },
        ))
        .id();
    add_or_attach_line(cmds, attached_lines, start, line, true);
    add_or_attach_line(cmds, attached_lines, end, line, false);
    line
}

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
