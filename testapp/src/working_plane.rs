use bevy::{
    color::palettes,
    ecs::system::{RunSystemOnce, SystemParam},
    prelude::*,
};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::ui_for_value;
use selo::prelude::Workplane;

use crate::{
    gizmos::GizmosExt, line::Line, point::Point, ring::Ring2D, state::AppState, triangle::Triangle,
};

pub struct WorkplanePlugin;

impl Plugin for WorkplanePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<ActiveWorkplane>()
            .register_type::<StoredWorkplane>()
            .register_type::<AttachedWorkplane>()
            .add_systems(Startup, spawn_initial_working_plane)
            .add_systems(
                Update,
                (ui_active, ui_inactive)
                    .chain()
                    .run_if(in_state(AppState::Workplane)),
            )
            .add_systems(Update, render_working_plane)
            .observe(add_working_plane::<Point>)
            .observe(add_working_plane::<Line>)
            .observe(add_working_plane::<Triangle>)
            .observe(add_working_plane::<Ring2D>)
            .observe(keep_active_working_plane_unique);
    }
}

#[derive(Debug, Clone, Component, Reflect, Deref, DerefMut)]
pub struct AttachedWorkplane(pub Workplane);

#[derive(Debug, Clone, Component, Default, Reflect)]
pub struct ActiveWorkplane;

#[derive(Debug, Clone, Component, Reflect, Deref, DerefMut)]
pub struct StoredWorkplane(pub Workplane);

#[derive(SystemParam)]
pub struct WorkplaneParams<'w, 's> {
    pub active: Query<'w, 's, &'static StoredWorkplane, With<ActiveWorkplane>>,
}

impl WorkplaneParams<'_, '_> {
    pub fn current(&self) -> Workplane {
        **self.active.single()
    }
}

fn add_working_plane<C: Component>(
    trigger: Trigger<OnAdd, C>,
    mut cmds: Commands,
    working_plane: WorkplaneParams,
) {
    cmds.entity(trigger.entity())
        .insert(AttachedWorkplane(working_plane.current()));
}

fn keep_active_working_plane_unique(
    trigger: Trigger<OnAdd, ActiveWorkplane>,
    mut cmds: Commands,
    other: Query<Entity, With<ActiveWorkplane>>,
) {
    other
        .iter()
        .filter(|&e| e != trigger.entity())
        .for_each(|entity| {
            cmds.entity(entity).remove::<ActiveWorkplane>();
        });
}

fn spawn_initial_working_plane(mut cmds: Commands) {
    cmds.spawn((
        Name::new("Initial Workplane"),
        ActiveWorkplane,
        StoredWorkplane(
            Workplane::from_three_points([Vec3::ZERO, Vec3::X, Vec3::Y]).hesse_normal_form(),
        ),
    ));
}

fn render_working_plane(mut gizmos: Gizmos, working_plane: WorkplaneParams) {
    let working_plane = working_plane.current();
    gizmos.plane_3d(
        working_plane.origin(),
        working_plane.normal(),
        palettes::basic::SILVER,
    );
    gizmos.plane_3d(Vec3::ZERO, Dir3::Z, palettes::basic::GREEN);
}

fn ui_active(world: &mut World) {
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    let mut q = world.query_filtered::<&StoredWorkplane, With<ActiveWorkplane>>();
    let mut wp = q.single(world).clone();
    let resp =
        egui::Window::new("Workplane Points").show(&ctx, |ui| ui_for_value(&mut wp, ui, world));
    if resp.is_some_and(|inner| inner.inner.is_some_and(|changed| changed)) {
        let mut q = world.query_filtered::<&mut StoredWorkplane, With<ActiveWorkplane>>();
        let mut current = q.single_mut(world);
        current.origin = wp.origin;
        current.plane.normal = Dir3::new_unchecked(wp.plane.normal.normalize());
    }
}

fn ui_inactive(world: &mut World) {
    enum Outcome {
        New,
        NewActive(Entity),
    }
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    let mut q = world
        .query_filtered::<(Entity, &Name), (With<StoredWorkplane>, Without<ActiveWorkplane>)>();
    let wp = q
        .iter(world)
        .map(|(entity, name)| (entity, name.to_string()))
        .collect::<Vec<_>>();
    let resp = egui::Window::new("New Workplanes")
        .show(&ctx, |ui| {
            for (e, name) in wp {
                if ui.button(name).clicked() {
                    return Some(Outcome::NewActive(e));
                }
            }
            if ui.button("+").clicked() {
                return Some(Outcome::New);
            }
            None
        })
        .map(|resp| resp.inner)
        .flatten()
        .flatten();

    match resp {
        Some(o) => match o {
            Outcome::New => {
                world.run_system_once(spawn_initial_working_plane);
            }
            Outcome::NewActive(e) => {
                world.commands().entity(e).insert(ActiveWorkplane);
            }
        },
        None => {}
    }
}
