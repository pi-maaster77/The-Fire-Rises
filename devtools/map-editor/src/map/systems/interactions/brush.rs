// devtools/map-editor/src/map/systems/interactions/brush.rs
use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use wasm_bindgen::prelude::wasm_bindgen;
use crate::{bridge::systems::selection::RenderUpdateTrigger, map::components::{ProvincePixelMap, ProvinceStateMap, Province}};

pub fn brush_system(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse: Res<ButtonInput<MouseButton>>,
    brush: Res<BrushSettings>,
    pixel_map: Res<ProvincePixelMap>,
    mut state_map: ResMut<ProvinceStateMap>,
    mut provinces: Query<&mut Province>,
) {
    // Si no estamos pintando o no hay click, no hacemos nada
    if !brush.is_painting || !mouse.pressed(MouseButton::Left) { return; }
    let Some(target_state) = &brush.active_state_id else { return };

    let Ok(window) = window_q.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return };

    if let Some(world_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        // Convertimos mundo a coordenadas de imagen
        let x = (world_pos.x + (pixel_map.width as f32 / 2.0)) as u32;
        let y = ((pixel_map.height as f32 / 2.0) - world_pos.y) as u32;

        if x < pixel_map.width && y < pixel_map.height {
            let idx = (y * pixel_map.width + x) as usize;
            if let Some(prov_id) = &pixel_map.data[idx] {
                // Si la provincia no tiene ese estado, la pintamos
                if state_map.0.get(prov_id) != Some(target_state) {
                    state_map.0.insert(prov_id.clone(), target_state.clone());
                    if let Some(mut province) = provinces.iter_mut().find(|p| p.id == *prov_id) {
                        province.state_id = target_state.clone();
                    }
                    commands.insert_resource(RenderUpdateTrigger);
                }
            }
        }
    }
}

#[wasm_bindgen]
unsafe extern "C" {
    #[wasm_bindgen(js_name = onProvinceAssigned)]
    fn on_province_assigned(prov_id: String, state_id: String);
}

#[derive(Resource, Default)]
pub struct BrushSettings {
    pub active_state_id: Option<String>,
    pub is_painting: bool,
}