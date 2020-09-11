use crate::systems::{camera, drone, unit};
use bevy::prelude::*;
use bevy_mod_picking::*;

mod bundles;
mod initialize;
mod systems;

fn main() {
    App::build()
        .add_resource(Msaa { samples: 4 })
        .add_resource(WindowDescriptor {
            title: "bavy".to_string(),
            width: 1600,
            height: 1600,
            ..Default::default()
        })
        .add_default_plugins()
        .add_plugin(PickingPlugin)
        .add_startup_system(initialize::setup.system())
        .add_plugin(drone::DronePlugin)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(unit::UnitPlugin)
        .run();
}
