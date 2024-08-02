use bevy::{
    input::common_conditions::{input_just_pressed, input_just_released},
    prelude::*,
};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::ui_for_state;
use bevy_mod_picking::prelude::*;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<AlgoState>()
            .register_type::<AppState>()
            .register_type::<AlgoState>()
            .register_type::<PreviousState>()
            .init_resource::<PreviousState>()
            .add_systems(
                Update,
                (main_state_ui, sub_state_ui.run_if(in_state(AppState::Play))),
            )
            .add_systems(
                Update,
                (
                    change_state(AppState::Point).run_if(input_just_pressed(KeyCode::KeyP)),
                    change_state(AppState::Line).run_if(input_just_pressed(KeyCode::KeyL)),
                    change_state(AppState::Play).run_if(input_just_pressed(KeyCode::Escape)),
                    use_prev_state.run_if(input_just_released(KeyCode::Space)),
                    change_state(AppState::Move).run_if(input_just_pressed(KeyCode::Space)),
                ),
            )
            .add_systems(
                Update,
                unselect_on_state_change.run_if(state_changed::<AppState>),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Default, Reflect, Resource)]
pub struct PreviousState(AppState);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Reflect)]
pub enum AppState {
    #[default]
    Play,
    Point,
    Line,
    Move,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SubStates, Reflect)]
#[source(AppState = AppState::Play)]
pub enum AlgoState {
    #[default]
    None,
    LineIntersection,
}

fn change_state(
    state: AppState,
) -> impl Fn(ResMut<NextState<AppState>>, Res<State<AppState>>, ResMut<PreviousState>) {
    move |mut next, now, mut prev| {
        next.set(state);
        **prev = *now.get();
    }
}

fn main_state_ui(world: &mut World) {
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    egui::Window::new("State").show(&ctx, |ui| {
        ui_for_state::<AppState>(world, ui);
    });
}

fn sub_state_ui(world: &mut World) {
    let mut q = world.query::<&mut EguiContext>();
    let ctx = q.single_mut(world).get_mut().clone();
    egui::SidePanel::left("AlgoState").show(&ctx, |ui| {
        ui_for_state::<AlgoState>(world, ui);
    });
}

fn unselect_on_state_change(mut selected: Query<&mut PickSelection>) {
    selected.iter_mut().for_each(|mut selection| {
        selection.is_selected = false;
    });
}

fn use_prev_state(prev: Res<PreviousState>, mut next: ResMut<NextState<AppState>>) {
    next.set(**prev);
}
