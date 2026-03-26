// devtools/map-editor/src/lib.rs

use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

mod map;
mod bridge; // Comunicación con JS

#[wasm_bindgen(start)]
pub fn run_app() {
    App::new()
				// devtools/map-editor/src/lib.rs
				.add_plugins(DefaultPlugins.set(WindowPlugin {
						primary_window: Some(Window {
								canvas: Some("#canvas".into()),
								resolution: bevy::window::WindowResolution::new(800.0, 600.0)
									.with_scale_factor_override(1.0),
								// En Bevy 0.13+ fit_canvas_to_parent ya no existe en Window, 
								// se maneja automáticamente o vía CSS/WindowResolution
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
    let mut camera = Camera2dBundle::default();
    
    // Si querés que el mapa se vea más grande de entrada, 
    // bajá este valor (0.5 es 2x zoom, 2.0 es 0.5x zoom)
    camera.projection.scale = 0.8; 
    
    commands.spawn(camera);
}