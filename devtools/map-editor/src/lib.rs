// devtools/map-editor/src/lib.rs

use bevy::{prelude::*, window::WindowResolution};
use wasm_bindgen::prelude::wasm_bindgen;

mod map;
mod bridge; // Comunicación con JS

#[wasm_bindgen(start)]
pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                canvas: Some("#canvas".into()),
                // Configuramos la resolución para que sea flexible
                resolution: WindowResolution::default()
                    .with_scale_factor_override(1.0),
                // Eliminamos la línea fit_canvas_to_parent que daba error
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.1, 0.1, 0.1)))
        .add_plugins(map::MapPlugin)
        .add_plugins(bridge::BridgePlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    // Dejamos la cámara por defecto por ahora para asegurar que se vea
    commands.spawn(Camera2dBundle::default());
}