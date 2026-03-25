// frontend/map/src/bridge/mod.rs

use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use serde::Serialize;

use crate::map::components::Province;

#[wasm_bindgen]
extern "C" {
    // Un solo punto de entrada en JS para todos los eventos
    #[wasm_bindgen(js_name = __BEVY_BRIDGE_INBOUND__)]
    fn send_to_js(event: JsValue);
}

#[derive(Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum GameEvent {
    #[serde(rename = "PROVINCE_SELECTED")]
    ProvinceSelected { id: u32, state_id: u32 },
    // Aquí irían ECONOMY_UPDATE, TICK, etc.
}

pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_clicks);
    }
}

fn handle_clicks(
    mouse: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    window_q: Query<&Window>,
    provinces_q: Query<(&Province, &GlobalTransform, &Sprite)>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = window_q.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(world_position) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) 
        {
            for (prov, transform, sprite) in provinces_q.iter() {
                let size = sprite.custom_size.unwrap_or(Vec2::ZERO) / 2.0;
                let pos = transform.translation().truncate();
                
                // Detección de click simple (AABB)
                if world_position.x > pos.x - size.x && world_position.x < pos.x + size.x &&
                   world_position.y > pos.y - size.y && world_position.y < pos.y + size.y {
                    
                    // Enviamos el evento a Vue
                    let event = GameEvent::ProvinceSelected { id: prov.id, state_id: prov.state_id };
                    if let Ok(js_val) = serde_wasm_bindgen::to_value(&event) {
                        send_to_js(js_val);
                    }
                    break;
                }
            }
        }
    }
}