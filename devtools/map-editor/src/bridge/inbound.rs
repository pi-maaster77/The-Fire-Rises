// devtools/map-editor/src/bridge/inbound.rs

use wasm_bindgen::prelude::wasm_bindgen;
use crate::bridge::state::{ACTIVE_REGION_UPDATE, BRUSH_UPDATE, EXTERNAL_SELECTION, MAP_IMAGE_DATA, STATE_ASSIGNMENTS};

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

#[wasm_bindgen]
pub fn update_brush_settings(active: bool, state_id: Option<String>) {
    if let Ok(mut brush) = BRUSH_UPDATE.lock() {
        *brush = Some((active, state_id));
    }
}

#[wasm_bindgen]
pub fn set_active_region(region_id: Option<String>) {
    if let Ok(mut guard) = ACTIVE_REGION_UPDATE.lock() {
        *guard = Some(region_id);
    }
}