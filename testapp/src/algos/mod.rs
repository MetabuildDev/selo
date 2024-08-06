mod algostate;
mod line_intersection;
mod polygon_expand;
mod polygon_triangulate;
mod straight_skeleton;
mod working_planes;

use bevy::{color::palettes, prelude::*};

use crate::state::AppState;

pub struct AlgoPlugin;

impl Plugin for AlgoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(algostate::AlgorithmStatePlugin)
            .add_plugins((
                line_intersection::LineIntersectionPlugin,
                polygon_triangulate::PolygonTriangulationPlugin,
                polygon_expand::PolygonExpandPlugin,
                straight_skeleton::PolygonSkeletonPlugin,
                working_planes::WorkingPlanePlugin,
            ))
            .add_systems(Update, draw_origin.run_if(in_state(AppState::Algorithms)));
    }
}

fn draw_origin(mut gizmos: Gizmos) {
    gizmos.sphere(Vec3::ZERO, Quat::default(), 0.05, palettes::basic::PURPLE);
}
