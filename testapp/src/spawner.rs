use bevy::{color::palettes, ecs::entity::EntityHashSet, prelude::*};
use bevy_inspector_egui::{bevy_egui::EguiContexts, egui};
use selo::{Geometry, IterPoints, Unembed};

use crate::{
    line::{AttachedLines, Line, LinePoint},
    parsing::{self, DynamicGeometries},
    point::Point,
    ring::{Ring2D, RingLine, RingPoint},
    triangle::{Triangle, TriangleLine, TrianglePoint},
    workplane::{ActiveWorkplane, StoredWorkplane},
};

pub struct SpawnerPlugin;

impl Plugin for SpawnerPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SpawnLine>()
            .add_event::<SpawnTriangle>()
            .add_event::<SpawnRing>()
            .add_systems(
                Update,
                (
                    spawn_line.run_if(on_event::<SpawnLine>),
                    spawn_triangle.run_if(on_event::<SpawnTriangle>),
                    spawn_ring.run_if(on_event::<SpawnRing>),
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
    mut ev_spawn_line: EventWriter<SpawnLine>,
    mut ev_spawn_triangle: EventWriter<SpawnTriangle>,
    mut ev_spawn_ring: EventWriter<SpawnRing>,
    mut prompt: Local<String>,
) {
    egui::Window::new("Spawn geometry").show(ctx.ctx_mut(), |ui| {
        ui.text_edit_multiline(&mut *prompt);

        ui.horizontal(|ui| {
            if ui.button("Submit").clicked() {
                let workplane = workplane.single().0;

                let mut spawn_geometry = |geometry: Geometry<Vec3>| {
                    match geometry {
                        Geometry::Line(line) => {
                            ev_spawn_line.send(SpawnLine(line));
                        }
                        Geometry::Triangle(triangle) => {
                            ev_spawn_triangle.send(SpawnTriangle(triangle));
                        }
                        Geometry::Ring(ring) => {
                            ev_spawn_ring.send(SpawnRing(ring));
                        }
                        Geometry::MultiRing(multi_ring) => {
                            for ring in multi_ring.0 {
                                ev_spawn_ring.send(SpawnRing(ring));
                            }
                        }
                        // TODO: Actual polygons
                        Geometry::Polygon(polygon) => {
                            for ring in polygon.iter_rings() {
                                ev_spawn_ring.send(SpawnRing(ring.clone()));
                            }
                        }
                        Geometry::MultiPolygon(multi_polygon) => {
                            for polygon in multi_polygon.0 {
                                for ring in polygon.iter_rings() {
                                    ev_spawn_ring.send(SpawnRing(ring.clone()));
                                }
                            }
                        }
                        // TODO: Handle others
                        _ => {}
                    }
                };

                match parsing::parse(&prompt) {
                    Ok(geometries) => {
                        info!("{:?}", geometries);
                        *prompt = String::new();
                        let geometries = match geometries {
                            DynamicGeometries::Dim2(g) => g
                                .into_iter()
                                .map(|g| match g {
                                    Geometry::Line(line) => Geometry::Line(line.unembed(workplane)),
                                    Geometry::LineString(line_string) => {
                                        Geometry::LineString(line_string.unembed(workplane))
                                    }
                                    Geometry::MultiLineString(multi_line_string) => {
                                        Geometry::MultiLineString(
                                            multi_line_string.unembed(workplane),
                                        )
                                    }
                                    Geometry::Triangle(triangle) => {
                                        Geometry::Triangle(triangle.unembed(workplane))
                                    }
                                    Geometry::Ring(ring) => Geometry::Ring(ring.unembed(workplane)),
                                    Geometry::MultiRing(multi_ring) => {
                                        Geometry::MultiRing(multi_ring.unembed(workplane))
                                    }
                                    Geometry::Polygon(polygon) => {
                                        Geometry::Polygon(polygon.unembed(workplane))
                                    }
                                    Geometry::MultiPolygon(multi_polygon) => {
                                        Geometry::MultiPolygon(multi_polygon.unembed(workplane))
                                    }
                                })
                                .collect(),
                            DynamicGeometries::Dim3(g) => g,
                        };
                        for g in geometries {
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
}

#[derive(Debug, Clone, Event)]
pub struct SpawnLine(pub selo::Line<Vec3>);

fn spawn_line(
    mut spawn_events: EventReader<SpawnLine>,
    mut cmds: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ids: Local<(usize, usize)>,
    mut attached_lines: Query<&mut AttachedLines, ()>,
) {
    spawn_events
        .read()
        .for_each(|SpawnLine(selo::Line([a, b]))| {
            let [a, b] = [a, b].map(|position| {
                spawn_point_inner(
                    *position,
                    &mut cmds,
                    &mut meshes,
                    &mut materials,
                    &mut ids.0,
                    |_id| LinePoint,
                )
            });

            spawn_line_inner(a, b, &mut cmds, &mut attached_lines, &mut ids.1, |_| {
                TriangleLine
            });
        });
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
                    &mut ids.1,
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
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Transform::from_translation(position),
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
