use bevy::{
    input::{
        common_conditions::{input_just_pressed, input_just_released},
        mouse::MouseWheel,
    },
    prelude::*,
};
use bevy_egui::{egui, EguiContext};
use bevy_inspector_egui::bevy_inspector::ui_for_state;
use bevy_mod_picking::prelude::*;
use strum::{EnumIter, IntoEnumIterator};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<AppState>()
            .add_sub_state::<AlgorithmState>()
            .register_type::<AppState>()
            .register_type::<AlgorithmState>()
            .register_type::<PreviousState>()
            .init_resource::<PreviousState>()
            .add_systems(
                Update,
                (
                    state_ui::<AppState>,
                    state_ui::<AlgorithmState>.run_if(in_state(AppState::Algorithms)),
                ),
            )
            .add_systems(
                Update,
                (
                    change_state(AppState::Point).run_if(input_just_pressed(KeyCode::KeyC)),
                    change_state(AppState::Line).run_if(input_just_pressed(KeyCode::KeyL)),
                    change_state(AppState::Triangle).run_if(input_just_pressed(KeyCode::KeyT)),
                    change_state(AppState::Polygon).run_if(input_just_pressed(KeyCode::KeyP)),
                    change_state(AppState::Algorithms).run_if(input_just_pressed(KeyCode::Escape)),
                    use_prev_state.run_if(input_just_released(KeyCode::Space)),
                    change_state(AppState::Move).run_if(input_just_pressed(KeyCode::Space)),
                ),
            )
            .add_systems(
                Update,
                unselect_everything.run_if(state_changed::<AppState>),
            )
            .add_systems(
                Update,
                next_algo_on_scroll
                    .run_if(state_exists::<AlgorithmState>.and_then(on_event::<MouseWheel>())),
            );
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Deref, DerefMut, Default, Reflect, Resource)]
pub struct PreviousState(AppState);

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States, Reflect)]
pub enum AppState {
    #[default]
    Algorithms,
    Point,
    Line,
    Triangle,
    Polygon,
    Move,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SubStates, Reflect, EnumIter)]
#[source(AppState = AppState::Algorithms)]
pub enum AlgorithmState {
    #[default]
    None,
    LineIntersection,
    PolygonTriangulate,
}

fn change_state(
    state: AppState,
) -> impl Fn(ResMut<NextState<AppState>>, Res<State<AppState>>, ResMut<PreviousState>) {
    move |mut next, now, mut prev| {
        next.set(state);
        **prev = *now.get();
    }
}

fn state_ui<S: bevy::state::state::FreelyMutableState + bevy::prelude::Reflect>(world: &mut World) {
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

fn use_prev_state(prev: Res<PreviousState>, mut next: ResMut<NextState<AppState>>) {
    next.set(**prev);
}

fn next_algo_on_scroll(
    mut ev_scroll: EventReader<MouseWheel>,
    mut next: ResMut<NextState<AlgorithmState>>,
    current: Res<State<AlgorithmState>>,
) {
    let total = ev_scroll.read().map(|ev| ev.y).sum::<f32>();
    let current = current.get();
    match total.partial_cmp(&0.0) {
        Some(std::cmp::Ordering::Less) => {
            next.set(
                AlgorithmState::iter()
                    .cycle()
                    .skip_while(|x| x != current)
                    .nth(1)
                    .unwrap(),
            );
        }
        Some(std::cmp::Ordering::Greater) => {
            next.set(
                AlgorithmState::iter()
                    .rev()
                    .cycle()
                    .skip_while(|x| x != current)
                    .nth(1)
                    .unwrap(),
            );
        }
        _ => {}
    }
}
