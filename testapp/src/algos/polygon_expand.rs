use bevy::{color::palettes, prelude::*};
use bevy_egui::{egui, EguiContext};
use math::buffer_polygon_glam;

use crate::polygon::PolygonParams;

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
    polygons: PolygonParams,
    expansion_factor: Res<PolygonExpansion>,
) {
    polygons
        .iter_polygons()
        // don't do that anymore and instead rotate to plane
        .map(|polygon| {
            polygon
                .into_iter()
                .map(|p| p.truncate())
                .collect::<Vec<_>>()
        })
        .flat_map(|polygon| buffer_polygon_glam(polygon, **expansion_factor))
        .for_each(|polygon| {
            polygon
                .windows(2)
                .map(|win| (win[0], win[1]))
                .chain(Some(()).and_then(|_| {
                    let first = polygon.first()?;
                    let last = polygon.last()?;
                    (first != last).then_some((*first, *last))
                }))
                .for_each(|(start, end)| {
                    gizmos.line(start.extend(0.0), end.extend(0.0), palettes::basic::RED);
                });
        });
}

fn ui(mut contexts: Query<&mut EguiContext>, mut val: ResMut<PolygonExpansion>) {
    egui::Window::new("Polygon Expansion Factor").show(contexts.single_mut().get_mut(), |ui| {
        ui.add(egui::DragValue::new(&mut **val).speed(0.005));
    });
}
