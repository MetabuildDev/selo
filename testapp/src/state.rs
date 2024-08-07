use bevy::{input::common_conditions::input_just_pressed, prelude::*};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::ui_for_state;
use bevy_mod_picking::prelude::*;

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
                    change_state(AppState::Polygon).run_if(input_just_pressed(KeyCode::KeyP)),
                    change_state(AppState::Algorithms).run_if(input_just_pressed(KeyCode::Escape)),
                    change_state(AppState::WorkingPlane).run_if(input_just_pressed(KeyCode::KeyW)),
                ),
            )
            .add_systems(
                Update,
                unselect_everything.run_if(state_changed::<AppState>),
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
    Polygon,
    WorkingPlane,
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

pub fn unselect_everything(mut selected: Query<&mut PickSelection>) {
    selected.iter_mut().for_each(|mut selection| {
        selection.is_selected = false;
    });
}
