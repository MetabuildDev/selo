use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_inspector_egui::bevy_egui::EguiContext;
use bevy_inspector_egui::bevy_inspector::ui_for_state;
use bevy_inspector_egui::egui;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .register_type::<AppState>()
            .add_systems(Update, state_ui::<AppState>)
            .add_systems(
                Update,
                (
                    change_state(AppState::Point).run_if(input_just_pressed(KeyCode::KeyC)),
                    change_state(AppState::Line).run_if(input_just_pressed(KeyCode::KeyL)),
                    change_state(AppState::Triangle).run_if(input_just_pressed(KeyCode::KeyT)),
                    change_state(AppState::Ring).run_if(input_just_pressed(KeyCode::KeyP)),
                    change_state(AppState::Algorithms).run_if(input_just_pressed(KeyCode::Escape)),
                    change_state(AppState::Workplane).run_if(input_just_pressed(KeyCode::KeyW)),
                ),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Reflect)]
pub enum AppState {
    #[default]
    Algorithms,
    Point,
    Line,
    Triangle,
    Ring,
    Workplane,
}

fn change_state(state: AppState) -> impl Fn(ResMut<NextState<AppState>>) {
    move |mut next| {
        next.set(state);
    }
}

pub fn state_ui<S: bevy::state::state::FreelyMutableState + bevy::prelude::Reflect>(
    world: &mut World,
) {
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    egui::Window::new(
        std::any::type_name::<S>()
            .split("::")
            .last()
            .unwrap_or_default(),
    )
    .show(&ctx, |ui| {
        ui_for_state::<S>(world, ui);
    });
}
