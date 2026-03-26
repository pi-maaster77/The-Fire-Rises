// devtools/map-editor/src/map/mod.rs

use bevy::prelude::*;

// frontend/map/src/map/mod.rs

pub mod components;
pub mod systems;

use self::systems::setup::spawn_map;
use self::systems::scanner::scanner_system;
use self::systems::interactions::handle_click;
use self::systems::render::render_map_sprite;
use crate::{
	bridge::{
		ScanTrigger, 
		check_external_selection, 
		check_load_image
	}, 
	map::systems::setup::handle_window_resize
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // Aquí es donde le decimos a Bevy que use la función
        app.add_systems(Startup, spawn_map);
        app.add_systems(Update, (
					check_load_image, 
					scanner_system.run_if(resource_exists::<ScanTrigger>), 
					handle_click, 
					render_map_sprite,
					check_external_selection,
					handle_window_resize
				));
    }
}