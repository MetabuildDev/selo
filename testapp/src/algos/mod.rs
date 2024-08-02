mod line_intersection;

use bevy::prelude::*;

pub struct AlgoPlugin;

impl Plugin for AlgoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(line_intersection::LineIntersectionPlugin);
    }
}
