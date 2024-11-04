use bevy::{color::palettes, ecs::entity::EntityHashSet, prelude::*};
use bevy_egui::{egui, EguiContexts};
use selo::{IterPoints, Unembed};

use crate::{
    line::{AttachedLines, Line},
    parsing::{self, Geometry},
    point::Point,
    ring::{Ring2D, RingLine, RingPoint},
    triangle::{Triangle, TriangleLine, TrianglePoint},
    workplane::{ActiveWorkplane, StoredWorkplane},
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
                    spawn_ui,
                ),
            );
    }
}

fn spawn_ui(
    mut cmds: Commands,
    mut ctx: EguiContexts,
    workplane: Query<&StoredWorkplane, With<ActiveWorkplane>>,
    q_geometry: Query<
        Entity,
        Or<(
            With<Triangle>,
            With<Ring2D>,
            With<RingPoint>,
            With<RingLine>,
        )>,
    >,
    mut ev_spawn_triangle: EventWriter<SpawnTriangle>,
    mut ev_spawn_ring: EventWriter<SpawnRing>,
    mut prompt: Local<String>,
) {
    egui::Window::new("spawn_ui").show(ctx.ctx_mut(), |ui| {
        ui.text_edit_multiline(&mut *prompt);

        ui.horizontal(|ui| {
            if ui.button("Submit").clicked() {
                let workplane = workplane.single().0;

                let mut spawn_geometry = |geometry| {
                    match geometry {
                        Geometry::Triangle(triangle) => {
                            ev_spawn_triangle.send(SpawnTriangle(triangle.unembed(workplane)));
                        }
                        Geometry::Ring(ring) => {
                            ev_spawn_ring.send(SpawnRing(ring.unembed(workplane)));
                        }
                        Geometry::MultiRing(multi_ring) => {
                            for ring in multi_ring.0 {
                                ev_spawn_ring.send(SpawnRing(ring.unembed(workplane)));
                            }
                        }
                        // TODO: Actual polygons
                        Geometry::Polygon(polygon) => {
                            for ring in polygon.iter_rings() {
                                ev_spawn_ring.send(SpawnRing(ring.unembed(workplane)));
                            }
                        }
                        Geometry::MultiPolygon(multi_polygon) => {
                            for polygon in multi_polygon.0 {
                                for ring in polygon.iter_rings() {
                                    ev_spawn_ring.send(SpawnRing(ring.unembed(workplane)));
                                }
                            }
                        }
                        // TODO: Handle others
                        _ => {}
                    }
                };

                match parsing::parse(&prompt) {
                    Ok(geometry) => {
                        *prompt = String::new();
                        for g in geometry {
                            spawn_geometry(g)
                        }
                    }
                    Err(e) => {
                        println!("Failed to parse:\n{e}");
                    }
                }
            }

            if ui.button("Despawn all").clicked() {
                for e in &q_geometry {
                    cmds.entity(e).despawn_recursive();
                }
            }
        })
    });
    // let mut q = world.query::<&mut EguiContext>();
    // let ctx = q.single_mut(world).get_mut().clone();
    // egui::Window::new(
    //     std::any::type_name::<S>()
    //         .split("::")
    //         .last()
    //         .unwrap_or_default(),
    // )
    // .show(&ctx, |ui| {
    //     ui_for_state::<S>(world, ui);
    // });
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
