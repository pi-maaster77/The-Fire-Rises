// devtools/map-editor/src/map/mod.rs

use bevy::prelude::*;

// frontend/map/src/map/mod.rs

pub mod components;
pub mod systems;

use self::systems::setup::spawn_map;
use self::systems::scanner::scanner_system;
use self::systems::render::render_map_sprite;
use crate::{
	bridge::systems::{assignments::sync_brush_settings, load_image::{ScanTrigger, check_load_image}, selection::check_external_selection}, 
	map::systems::{camera::camera_control_system, interactions::InteractionsPlugin, setup::handle_window_resize}
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        // Aquí es donde le decimos a Bevy que use la función
        app
				.init_resource::<components::ProvinceStateMap>() 
				.init_resource::<components::CameraConfig>()
				.add_systems(Startup, spawn_map)
        .add_systems(Update, (
					check_load_image, 
					scanner_system.run_if(resource_exists::<ScanTrigger>),
					render_map_sprite,
					check_external_selection,
					handle_window_resize,
					sync_brush_settings,
					camera_control_system,
				))
				.add_plugins(InteractionsPlugin);
    }
}