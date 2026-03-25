// frontend/map/src/map/mod.rs

use bevy::prelude::*;

// frontend/map/src/map/mod.rs

pub mod components;
pub mod systems;

use self::systems::{
    setup::spawn_map,
    image_processing::process_map_image,
};
use components::ProvinceMap;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // Aquí es donde le decimos a Bevy que use la función
        app.insert_resource(ProvinceMap::new(800, 600))
           .add_systems(Startup, spawn_map)
           .add_systems(Update, process_map_image);
    }
}