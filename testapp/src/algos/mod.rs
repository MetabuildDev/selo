mod algostate;
mod line_intersection;
mod polygon_expand;
mod polygon_triangulate;
mod straight_skeleton;

use bevy::prelude::*;

pub struct AlgoPlugin;

impl Plugin for AlgoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(algostate::AlgorithmStatePlugin)
            .add_plugins((
                line_intersection::LineIntersectionPlugin,
                polygon_triangulate::PolygonTriangulationPlugin,
                polygon_expand::PolygonExpandPlugin,
                straight_skeleton::PolygonSkeletonPlugin,
            ));
    }
}
