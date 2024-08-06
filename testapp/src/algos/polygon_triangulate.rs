use bevy::{color::palettes, prelude::*};
use math::triangulate_glam;

use crate::polygon::PolygonParams;

use super::algostate::AlgorithmState;

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
        // don't do that anymore and instead rotate to plane
        .map(|polygon| {
            polygon
                .into_iter()
                .map(|p| p.truncate())
                .collect::<Vec<_>>()
        })
        .flat_map(|polygon| triangulate_glam(polygon))
        .for_each(|[a, b, c]| {
            gizmos.primitive_2d(
                &Triangle2d::new(a, b, c),
                Vec2::ZERO,
                0.0,
                palettes::basic::RED,
            );
        });
}
