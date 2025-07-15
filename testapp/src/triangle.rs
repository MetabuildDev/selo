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

pub struct TrianglePlugin;

impl Plugin for TrianglePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<Triangle>()
            .register_type::<TrianglePoint>()
            .register_type::<TriangleMid>()
            .register_type::<TriangleLine>()
            .add_systems(
                Update,
                (
                    spawn_point
                        .pipe(triangle_point)
                        .pipe(triangle_start)
                        .pipe(drop_system)
                        .run_if(not(any_with_component::<TriangleStart>)),
                    spawn_point
                        .pipe(triangle_point)
                        .pipe(triangle_mid)
                        .pipe(drop_system)
                        .run_if(
                            any_with_component::<TriangleStart>
                                .and(not(any_with_component::<TriangleMid>)),
                        ),
                    spawn_point
                        .pipe(triangle_point)
                        .pipe(triangle_end)
                        .pipe(construct_lines)
                        .pipe(construct_triangle)
                        .pipe(drop_system)
                        .run_if(any_with_component::<TriangleMid>),
                )
                    .run_if(
                        in_state(AppState::Triangle).and(input_just_pressed(MouseButton::Left)),
                    ),
            )
            .add_systems(
                Update,
                (
                    render_triangles,
                    render_triangle_construction.run_if(
                        in_state(AppState::Triangle).and(any_with_component::<TriangleStart>),
                    ),
                ),
            )
            .add_systems(OnExit(AppState::Triangle), cleanup_unfinished);
    }
}

#[derive(Debug, Clone, Component, Reflect)]
pub struct Triangle {
    pub a: Entity,
    pub b: Entity,
    pub c: Entity,
}

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct TrianglePoint;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct TriangleStart;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct TriangleMid;

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct TriangleLine;

#[derive(SystemParam)]
pub struct TriangleParams<'w, 's> {
    triangles: Query<'w, 's, (Entity, &'static Triangle, &'static AttachedWorkplane)>,
    points: Query<'w, 's, &'static GlobalTransform, With<Point>>,
}

impl TriangleParams<'_, '_> {
    #[allow(unused)]
    pub fn iter_entities(&self) -> impl Iterator<Item = (Entity, [Entity; 3])> + '_ {
        self.triangles
            .iter()
            .map(|(entity, triangle, _)| (entity, [triangle.a, triangle.b, triangle.c]))
    }

    pub fn iter_just_triangles(&self) -> impl Iterator<Item = selo::Triangle<Vec3>> + '_ {
        self.iter_triangles().map(|(triangle, _)| triangle)
    }

    pub fn iter_triangles(&self) -> impl Iterator<Item = (selo::Triangle<Vec3>, Workplane)> + '_ {
        self.triangles.iter().filter_map(|(_, triangle, wp)| {
            let points = self
                .points
                .get_many([triangle.a, triangle.b, triangle.c])
                .map(|poss| selo::Triangle(poss.map(|pos| pos.translation())))
                .ok()?;
            Some((points, **wp))
        })
    }
}

fn triangle_point(In(entity): In<Entity>, mut cmds: Commands) -> Entity {
    cmds.entity(entity).insert(TrianglePoint).id()
}

fn triangle_start(In(entity): In<Entity>, mut cmds: Commands) -> Entity {
    cmds.entity(entity).insert(TriangleStart).id()
}

fn triangle_mid(In(entity): In<Entity>, mut cmds: Commands) -> Entity {
    cmds.entity(entity).insert(TriangleMid).id()
}

fn triangle_end(
    In(entity): In<Entity>,
    mut cmds: Commands,
    start: Query<Entity, With<TriangleStart>>,
    mid: Query<Entity, With<TriangleMid>>,
) -> [(Entity, Entity); 3] {
    let start = start.single().unwrap();
    let mid = mid.single().unwrap();
    cmds.entity(start).remove::<TriangleStart>();
    cmds.entity(mid).remove::<TriangleMid>();
    let end = entity;
    [(start, mid), (mid, end), (end, start)]
}

fn construct_triangle(
    In(lines): In<[(Entity, (Entity, Entity)); 3]>,
    mut cmds: Commands,
    mut triangle_id: Local<usize>,
) -> Entity {
    lines.iter().for_each(|(line, _)| {
        cmds.entity(*line).insert(TriangleLine);
    });
    let [a, b, c] = lines.map(|(_, (p, _))| p);
    *triangle_id += 1;
    cmds.spawn((
        Name::new(format!("Triangle {n}", n = *triangle_id)),
        Triangle { a, b, c },
    ))
    .id()
}

fn render_triangle_construction(
    mut gizmos: Gizmos,
    start: Query<&GlobalTransform, With<TriangleStart>>,
    mid: Query<&GlobalTransform, With<TriangleMid>>,
    pointer: PointerParams,
    workplane: WorkplaneParams,
) {
    let pointer_pos = pointer
        .world_position_3d(workplane.current())
        .unwrap_or_default();
    let start = start.single().unwrap().translation();
    let mid = mid.single().map(|p| p.translation());

    [(start, pointer_pos)]
        .into_iter()
        .chain(
            mid.map(|mid| [(start, mid), (mid, pointer_pos)])
                .into_iter()
                .flatten(),
        )
        .for_each(|(a, b)| {
            gizmos.line(a, b, palettes::basic::TEAL);
        });
}

fn render_triangles(mut gizmos: Gizmos, triangles: TriangleParams) {
    triangles
        .iter_just_triangles()
        .for_each(|selo::Triangle([a, b, c])| {
            gizmos.primitive_3d(
                &Triangle3d::new(a, b, c),
                Isometry3d::new(Vec3::ZERO, Quat::default()),
                palettes::basic::TEAL,
            );
        })
}

fn cleanup_unfinished(
    mut cmds: Commands,
    unfinished: Query<Entity, Or<(With<TriangleStart>, With<TriangleMid>)>>,
) {
    unfinished.iter().for_each(|entity| {
        cmds.entity(entity).despawn();
    });
}
