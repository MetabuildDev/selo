use bevy::{color::palettes, ecs::entity::EntityHashSet, prelude::*};

use crate::{
    line::{AttachedLines, Line},
    point::Point,
    triangle::{Triangle, TriangleLine, TrianglePoint},
};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnTriangle>()
            .add_systems(Update, spawn_triangle.run_if(on_event::<SpawnTriangle>()));
    }
}

#[derive(Debug, Clone, Event)]
pub struct SpawnTriangle {
    pub a: Vec3,
    pub b: Vec3,
    pub c: Vec3,
}

fn spawn_triangle(
    mut spawn_events: EventReader<SpawnTriangle>,

    mut cmds: Commands,

    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,

    mut ids: Local<(usize, usize, usize)>,

    mut attached_lines: Query<&mut AttachedLines, ()>,
) {
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

    spawn_events.read().for_each(|SpawnTriangle { a, b, c }| {
        let [a, b, c] = [a, b, c].map(|position| {
            ids.0 += 1;

            let name = Name::new(format!("Point {n}", n = ids.0));

            let mesh = meshes.add(Circle::new(0.025));
            let material = materials.add(StandardMaterial::from_color(Color::from(
                palettes::basic::WHITE,
            )));

            cmds.spawn((
                Point,
                TrianglePoint,
                name,
                MaterialMeshBundle {
                    mesh,
                    material,
                    transform: Transform::from_translation(*position),
                    ..Default::default()
                },
            ))
            .id()
        });
        let [_ab, _bc, _ca] = [(a, b), (b, c), (c, a)].map(|(start, end)| {
            ids.1 += 1;
            let line = cmds
                .spawn((
                    Name::new(format!("Line {n}", n = ids.1)),
                    TriangleLine,
                    Line { start, end },
                ))
                .id();
            add_or_attach_line(&mut cmds, &mut attached_lines, start, line, true);
            add_or_attach_line(&mut cmds, &mut attached_lines, end, line, false);
            line
        });

        ids.2 += 1;

        cmds.spawn((
            Name::new(format!("Triangle {n}", n = ids.2)),
            Triangle { a, b, c },
        ));
    });
}
