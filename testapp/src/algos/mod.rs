mod algostate;
mod line_intersection;
mod polygon_boolops;
mod polygon_expand;
mod polygon_triangulate;
mod stitch_polygons;
mod straight_skeleton;
mod workplanes;

use bevy::{color::palettes, prelude::*};

pub struct AlgoPlugin;

impl Plugin for AlgoPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(algostate::AlgorithmStatePlugin)
            .add_plugins((
                line_intersection::LineIntersectionPlugin,
                polygon_triangulate::PolygonTriangulationPlugin,
                polygon_expand::PolygonExpandPlugin,
                straight_skeleton::PolygonSkeletonPlugin,
                workplanes::WorkplanePlugin,
                stitch_polygons::StitchTrianglesPlugin,
                polygon_boolops::PolygonBoolopsPlugin,
            ))
            .add_systems(Update, draw_origin);
    }
}

fn draw_origin(mut gizmos: Gizmos) {
    gizmos.sphere(
        Isometry3d::new(Vec3::ZERO, Quat::default()),
        0.05,
        palettes::basic::PURPLE,
    );
}
