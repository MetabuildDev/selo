use bevy::{color::palettes, prelude::*};
use bevy_egui::{egui, EguiContext};
use itertools::Itertools;
use math::{buffer_polygon_glam, primitives::Ring};

use crate::ring::RingParams;

use super::algostate::AlgorithmState;

pub struct PolygonExpandPlugin;

impl Plugin for PolygonExpandPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<PolygonExpansion>()
            .add_systems(
                OnEnter(AlgorithmState::PolygonExpansion),
                |mut cmds: Commands| {
                    cmds.init_resource::<PolygonExpansion>();
                },
            )
            .add_systems(
                OnExit(AlgorithmState::PolygonExpansion),
                |mut cmds: Commands| {
                    cmds.remove_resource::<PolygonExpansion>();
                },
            )
            .add_systems(
                Update,
                (
                    render_polygon_expansion,
                    ui.run_if(resource_exists::<PolygonExpansion>),
                )
                    .run_if(in_state(AlgorithmState::PolygonExpansion)),
            );
    }
}

#[derive(Debug, Clone, Resource, Deref, DerefMut, Reflect, Default)]
struct PolygonExpansion(f64);

fn render_polygon_expansion(
    mut gizmos: Gizmos,
    rings: RingParams,
    expansion_factor: Res<PolygonExpansion>,
) {
    rings
        .iter_rings()
        .chunk_by(|(_, wp)| *wp)
        .into_iter()
        .for_each(|(wp, group)| {
            let (proj, inj) = wp.xy_projection_injection();
            group
                .into_iter()
                .map(|(ring, _)| {
                    Ring::new(
                        ring.into_iter()
                            .map(|p| proj.transform_point(p).truncate())
                            .collect::<Vec<_>>(),
                    )
                })
                .flat_map(|ring| buffer_polygon_glam(ring.to_polygon(), **expansion_factor).0)
                .for_each(|polygon| {
                    polygon
                        .lines()
                        .map(|line| line.0.map(|p| inj.transform_point(p.extend(0.0))))
                        .for_each(|line| {
                            gizmos.line(line[0], line[1], palettes::basic::RED);
                        });
                });
        })
}

fn ui(mut contexts: Query<&mut EguiContext>, mut val: ResMut<PolygonExpansion>) {
    egui::Window::new("Polygon Expansion Factor").show(contexts.single_mut().get_mut(), |ui| {
        ui.add(egui::DragValue::new(&mut **val).speed(0.005));
    });
}
