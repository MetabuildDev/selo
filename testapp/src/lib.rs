mod algos;
mod camera;
mod gizmos;
mod line;
mod point;
mod pointer;
mod ring;
mod spawner;
mod state;
mod triangle;
mod working_plane;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_mod_picking::prelude::*;

pub fn run() {
    let mut app = App::new();

    // plugins
    app.add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default())
        .add_plugins(DefaultPickingPlugins)
        .insert_resource(DebugPickingMode::Normal);

    app.add_plugins((
        state::StatePlugin,
        camera::CameraPlugin,
        point::PointPlugin,
        line::LinePlugin,
        triangle::TrianglePlugin,
        ring::RingPlugin,
        algos::AlgoPlugin,
        working_plane::WorkingPlanePlugin,
        spawner::SpawnerPlugin,
    ));

    app.run();
}

pub fn drop_system<I>(_: In<I>) {}
