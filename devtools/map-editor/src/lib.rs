// devtools/map-editor/src/lib.rs

use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

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
        // Nuestros Plugins personalizados (Escalabilidad)
        .add_plugins(map::MapPlugin)
        .add_systems(Startup, setup_camera)
        
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
