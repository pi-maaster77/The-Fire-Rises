// devtools/map-editor/src/bridge/mod.rs

use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use std::sync::Mutex;
use serde_json;
use crate::map::components::{MapImage, ProvincePixelMap};

static MAP_IMAGE_DATA: Mutex<Option<(Vec<u8>, u32, u32)>> = Mutex::new(None);

#[derive(Resource)]
pub struct ScanTrigger;

#[derive(Resource)]
pub struct RenderUpdateTrigger;

#[wasm_bindgen]
pub fn load_map_image(data: &[u8], width: u32, height: u32) {
    *MAP_IMAGE_DATA.lock().unwrap() = Some((data.to_vec(), width, height));
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
