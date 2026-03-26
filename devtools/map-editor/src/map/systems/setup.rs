// devtools/map-editor/src/map/systems/setup.rs

use bevy::{

	core_pipeline::core_2d::Camera2d, ecs::{event::EventReader, query::With, system::{Commands, Query}}, transform::components::Transform, window::WindowResized

};
use crate::map::components::{MapImage, ProvincePixelMap, SelectedProvinceId};

pub fn spawn_map(mut commands: Commands) {
    commands.insert_resource(MapImage {
        width: 0,
        height: 0,
        data: vec![],
    });
    commands.insert_resource(ProvincePixelMap {
        width: 0,
        height: 0,
        data: vec![],
    });
		commands.insert_resource(SelectedProvinceId(None));
}

pub fn handle_window_resize(
    mut resize_reader: EventReader<WindowResized>,
    mut query_camera: Query<&mut Transform, With<Camera2d>>,
) {
    for e in resize_reader.read() {
        // Cuando el canvas de JS cambia de tamaño, 
        // Bevy recibe este evento automáticamente.
        if let Ok(mut transform) = query_camera.get_single_mut() {
            // Opcional: Podés ajustar el zoom de la cámara aquí 
            // para que el mapa siempre quepa entero.
        }
    }
}