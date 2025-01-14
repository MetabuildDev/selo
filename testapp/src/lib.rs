mod algos;
mod camera;
mod gizmos;
mod line;
mod parsing;
mod point;
mod pointer;
mod ring;
mod spawner;
mod state;
mod triangle;
mod workplane;

use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub fn run() {
    let mut app = App::new();

    // plugins
    app.add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::default());

    app.add_plugins((
        state::StatePlugin,
        camera::CameraPlugin,
        point::PointPlugin,
        line::LinePlugin,
        triangle::TrianglePlugin,
        ring::RingPlugin,
        algos::AlgoPlugin,
        workplane::WorkplanePlugin,
        spawner::SpawnerPlugin,
    ));

    app.run();
}

pub fn drop_system<I>(_: In<I>) {}
