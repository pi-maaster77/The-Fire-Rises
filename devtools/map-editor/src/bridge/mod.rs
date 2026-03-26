// devtools/map-editor/src/bridge/mod.rs

use bevy::prelude::*;
use once_cell::sync::Lazy;
use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use serde_json;
use crate::map::components::{MapImage, Province, ProvincePixelMap, SelectedProvinceId};
use lazy_static::lazy_static; // Asegurate de tener esta crate o usá Mutex::new(None) directamente

static MAP_IMAGE_DATA: Mutex<Option<(Vec<u8>, u32, u32)>> = Mutex::new(None);
lazy_static! {
    static ref EXTERNAL_SELECTION: Mutex<Option<String>> = Mutex::new(None);
}
static STATE_ASSIGNMENTS: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| Mutex::new(Vec::new()));

#[derive(Resource)]
pub struct ScanTrigger;

#[derive(Resource)]
pub struct RenderUpdateTrigger;

#[wasm_bindgen]
pub fn load_map_image(data: &[u8], width: u32, height: u32) {
    *MAP_IMAGE_DATA.lock().unwrap() = Some((data.to_vec(), width, height));
}

#[wasm_bindgen]
pub fn select_province_by_id(id: String) {
    if let Ok(mut guard) = EXTERNAL_SELECTION.lock() {
        *guard = Some(id);
    }
}

#[wasm_bindgen]
pub fn assign_province_to_state(province_id: String, state_id: String) {
    if let Ok(mut assignments) = STATE_ASSIGNMENTS.lock() {
        assignments.push((province_id, state_id));
    }
}

pub fn send_to_vue(event_type: &str, payload: &serde_json::Value) {
    let payload_str = serde_json::to_string(payload).unwrap();
    let js_code = format!("if (window.__BEVY_BRIDGE_INBOUND__) {{ window.__BEVY_BRIDGE_INBOUND__({{type: '{}', payload: {}}}); }}", event_type, payload_str);
    js_sys::eval(&js_code).unwrap();
}

pub fn check_load_image(
    mut commands: Commands,
    mut map_image: Option<ResMut<MapImage>>,
    mut pixel_map: Option<ResMut<ProvincePixelMap>>,
) {
    if let Some((data, width, height)) = MAP_IMAGE_DATA.lock().unwrap().take() {
        if let (Some(map_image), Some(pixel_map)) = (map_image.as_mut(), pixel_map.as_mut()) {
            map_image.width = width;
            map_image.height = height;
            map_image.data = data;
            pixel_map.width = width;
            pixel_map.height = height;
            pixel_map.data = vec![None; (width * height) as usize];
            commands.insert_resource(ScanTrigger);
        }
    }
}

pub fn check_external_selection(
    mut commands: Commands,
    mut selected_res: ResMut<SelectedProvinceId>,
) {
    if let Ok(mut guard) = EXTERNAL_SELECTION.lock() {
        if let Some(id) = guard.take() {
            // Actualizamos el recurso y disparamos el repintado (el borde dorado)
            selected_res.0 = Some(id);
            commands.insert_resource(RenderUpdateTrigger);
        }
    }
}

pub fn sync_state_assignments(
    mut commands: Commands,
    mut provinces: Query<&mut Province>,
) {
    if let Ok(mut assignments) = STATE_ASSIGNMENTS.lock() {
        for (prov_id, state_id) in assignments.drain(..) {
            for mut province in provinces.iter_mut() {
                if province.id == prov_id {
                    province.state_id = state_id.clone();
                    // Trigger para repintar el mapa
                    commands.insert_resource(RenderUpdateTrigger);
                }
            }
        }
    }
}