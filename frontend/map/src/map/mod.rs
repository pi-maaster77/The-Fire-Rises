// frontend/map/src/map/mod.rs

use bevy::prelude::*;

// frontend/map/src/map/mod.rs

pub mod components;
pub mod systems;

use self::systems::spawn_map;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // Aquí es donde le decimos a Bevy que use la función
        app.add_systems(Startup, spawn_map);
    }
}