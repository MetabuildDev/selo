use bevy::{
    input::{common_conditions::input_pressed, mouse::MouseWheel},
    prelude::*,
};
use strum::{EnumIter, IntoEnumIterator};

use crate::state::{state_ui, AppState};

pub struct AlgorithmStatePlugin;

impl Plugin for AlgorithmStatePlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<AlgorithmState>()
            .add_sub_state::<AlgorithmState>()
            .add_systems(
                Update,
                state_ui::<AlgorithmState>.run_if(in_state(AppState::Algorithms)),
            )
            .add_systems(
                Update,
                next_algo_on_scroll.run_if(
                    state_exists::<AlgorithmState>
                        .and_then(input_pressed(KeyCode::ControlLeft))
                        .and_then(on_event::<MouseWheel>()),
                ),
            );
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, SubStates, Reflect, EnumIter)]
#[source(AppState = AppState::Algorithms)]
pub enum AlgorithmState {
    #[default]
    None,
    LineIntersection,
    PolygonTriangulate,
    StraightSkeleton,
    PolygonExpansion,
    WorkplaneNormalization,
    WorkplaneTransform,
    StitchTriangles,
    PolygonBoolops,
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
