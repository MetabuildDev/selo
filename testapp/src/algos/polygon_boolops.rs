#![allow(unused)]
use bevy::{color::palettes, input::common_conditions::input_just_pressed, prelude::*};
use itertools::Itertools;
use selo::{Embed, Unembed};

use crate::{
    ring::{Ring2D, RingLine, RingParams, RingPoint},
    spawner::SpawnRing,
};

use super::algostate::AlgorithmState;

pub struct PolygonBoolopsPlugin;

// FIXME: re-enable
impl Plugin for PolygonBoolopsPlugin {
    fn build(&self, app: &mut App) {
        // app.add_systems(
        //     Update,
        //     (
        //         render_polygon_union,
        //         do_unioning.run_if(input_just_pressed(KeyCode::Enter)),
        //     )
        //         .run_if(in_state(AlgorithmState::PolygonBoolops)),
        // );
    }
}

// fn render_polygon_union(mut gizmos: Gizmos, rings: RingParams) {
//     rings
//         .iter_rings()
//         .chunk_by(|(_, wp)| *wp)
//         .into_iter()
//         .for_each(|(wp, group)| {
//             let rings_projected = group
//                 .into_iter()
//                 .map(|(ring, _)| ring.embed(wp))
//                 .collect::<Vec<_>>();
//             // boolops_union_glam(rings_projected)
//             //     .0
//             //     .into_iter()
//             //     .for_each(|polygon| {
//             //         polygon.unembed(wp).lines().for_each(|line| {
//             //             gizmos.line(line.src(), line.dst(), palettes::basic::RED);
//             //         });
//             //     });
//         });
// }

// fn do_unioning(
//     mut cmds: Commands,
//     mut spawn_rings: EventWriter<SpawnRing>,
//     rings: RingParams,
//     entities: Query<Entity, Or<(With<Ring2D>, With<RingLine>, With<RingPoint>)>>,
// ) {
// spawn_rings.send_batch(
//     rings
//         .iter_rings()
//         .chunk_by(|(_, wp)| *wp)
//         .into_iter()
//         .flat_map(|(wp, group)| {
//             let polygons_projected = group
//                 .into_iter()
//                 .map(|(ring, _)| selo::Ring::embed(&ring, wp))
//                 .collect::<Vec<_>>();
// boolops_union_glam(polygons_projected)
//     .0
//     .into_iter()
//     .map(move |polygon| {
//         let exterior = polygon.exterior().unembed(wp);
//         SpawnRing(exterior)
//     })
// }),
// );
//
// entities.iter().for_each(|entity| {
//     cmds.entity(entity).despawn_recursive();
// });
// }
