// frontend/map/src/lib.rs

use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

use crate::map::systems::interactions::{handle_map_click::handle_map_click, highlight_selected_province::highlight_selected_province, highlight_state_border::highlight_state_border};

mod map;
mod bridge; // Comunicación con JS

#[wasm_bindgen(start)]
pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#canvas".into()),
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        // Recursos globales
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_systems(Update, handle_map_click)
				.add_systems(Update, highlight_selected_province)
				.add_systems(Update, highlight_state_border)
        // Nuestros Plugins personalizados (Escalabilidad)
        .add_plugins(map::MapPlugin)
        .add_plugins(bridge::BridgePlugin)
        
        .run();
}