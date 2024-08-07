use bevy::{color::palettes, ecs::system::SystemParam, prelude::*};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::ui_for_value;
use math::prelude::WorkingPlane;

use crate::state::AppState;

pub struct WorkingPlanePlugin;

impl Plugin for WorkingPlanePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ActiveWorkingPlane>()
            .register_type::<StoredWorkingPlane>()
            .register_type::<AttachedWorkingPlane>()
            .add_systems(Startup, spawn_initial_working_plane)
            .add_systems(Update, ui.run_if(in_state(AppState::WorkingPlane)))
            .add_systems(Update, render_working_plane);
    }
}

#[derive(Debug, Clone, Component, Reflect, Deref, DerefMut)]
pub struct AttachedWorkingPlane(pub WorkingPlane);

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct ActiveWorkingPlane;

#[derive(Debug, Clone, Component, Reflect, Deref, DerefMut)]
pub struct StoredWorkingPlane(pub WorkingPlane);

#[derive(SystemParam)]
pub struct WorkingPlaneParams<'w, 's> {
    pub active: Query<'w, 's, &'static StoredWorkingPlane, With<ActiveWorkingPlane>>,
}

impl WorkingPlaneParams<'_, '_> {
    pub fn current(&self) -> WorkingPlane {
        **self.active.single()
    }
}

pub fn with_working_plane(
    In(entity): In<Entity>,
    mut cmds: Commands,
    working_plane: WorkingPlaneParams,
) -> Entity {
    cmds.entity(entity)
        .insert(AttachedWorkingPlane(working_plane.current()))
        .id()
}

pub fn lines_with_working_plane<const N: usize>(
    In(entities): In<[(Entity, (Entity, Entity)); N]>,
    mut cmds: Commands,
    working_plane: WorkingPlaneParams,
) -> [(Entity, (Entity, Entity)); N] {
    entities.map(|inp @ (entity, _)| {
        cmds.entity(entity)
            .insert(AttachedWorkingPlane(working_plane.current()));
        inp
    })
}

fn spawn_initial_working_plane(mut cmds: Commands) {
    cmds.spawn((
        Name::new("Initial Working Plane"),
        ActiveWorkingPlane,
        StoredWorkingPlane(
            WorkingPlane::from_three_points([Vec3::ZERO, Vec3::X, Vec3::Y]).hesse_normal_form(),
        ),
    ));
}

fn render_working_plane(mut gizmos: Gizmos, working_plane: WorkingPlaneParams) {
    let working_plane = working_plane.current();
    gizmos
        .primitive_3d(
            &Plane3d {
                normal: working_plane.normal(),
                half_size: Vec2::splat(1000.0),
            },
            working_plane.origin(),
            Quat::default(),
            palettes::basic::SILVER,
        )
        .segment_count(100)
        .segment_length(0.1)
        .axis_count(8);

    gizmos
        .primitive_3d(
            &Plane3d {
                normal: Dir3::Z,
                half_size: Vec2::splat(1000.0),
            },
            Vec3::ZERO,
            Quat::default(),
            palettes::basic::GREEN,
        )
        .segment_count(100)
        .segment_length(0.1)
        .axis_count(8);
}

fn ui(world: &mut World) {
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    let mut q = world.query_filtered::<&StoredWorkingPlane, With<ActiveWorkingPlane>>();
    let mut wp = q.single(world).clone();
    let resp =
        egui::Window::new("Working Plane Points").show(&ctx, |ui| ui_for_value(&mut wp, ui, world));
    if resp.is_some_and(|inner| inner.inner.is_some_and(|changed| changed)) {
        let mut q = world.query_filtered::<&mut StoredWorkingPlane, With<ActiveWorkingPlane>>();
        let mut current = q.single_mut(world);
        current.origin = wp.origin;
        current.plane.normal = Dir3::new_unchecked(wp.plane.normal.normalize());
    }
}
