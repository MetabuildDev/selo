use bevy::{color::palettes, prelude::*};
use math::triangulate_glam;

use crate::{polygon::PolygonParams, state::AlgorithmState};

pub struct PolygonTriangulationPlugin;

impl Plugin for PolygonTriangulationPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            render_triangulation.run_if(in_state(AlgorithmState::PolygonTriangulate)),
        );
    }
}

fn render_triangulation(mut gizmos: Gizmos, polygons: PolygonParams) {
    polygons
        .iter_polygons()
        .flat_map(|polygon| triangulate_glam(polygon))
        .for_each(|[a, b, c]| {
            gizmos.primitive_2d(
                &Triangle2d::new(a, b, c),
                Vec2::ZERO,
                0.0,
                palettes::basic::TEAL,
            );
        });
}
