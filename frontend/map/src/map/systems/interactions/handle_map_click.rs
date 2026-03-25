// frontend/map/src/map/systems/interactions/handle_map_click.rs

use bevy::

{
	ecs::system::Commands, 
	math::Vec2, 
	prelude::*,
	sprite::{Sprite}
};

use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use serde::Serialize;

use crate::map::{components::{Province, Selected, State}};

#[wasm_bindgen]
extern "C" {
    // Un solo punto de entrada en JS para todos los eventos
    #[wasm_bindgen(js_name = __BEVY_BRIDGE_INBOUND__)]
    fn send_to_js(event: JsValue);
}

#[derive(Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ToVueEvent {
    #[serde(rename = "PROVINCE_SELECTED")]
    ProvinceSelected { province: Province, state: State },
    #[serde(rename = "PROVINCES_LIST")]
    ProvincesList { provinces: Vec<String> },
}

pub fn handle_map_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    window_q: Query<&Window>,
    provinces_q: Query<(Entity, &Province, Option<&Selected>)>,
    states_q: Query<&State>,
    province_map: Res<ProvinceMap>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = window_q.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(world_pos) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
        {
            // Convert world coordinates to image coordinates
            let image_x = (world_pos.x + province_map.width as f32 / 2.0) as u32;
            let image_y = (province_map.height as f32 / 2.0 - world_pos.y) as u32;

            if image_x < province_map.width && image_y < province_map.height {
                if let Some(province_id) = province_map.get(image_x, image_y) {
                    // Clear previous selection
                    for (entity, _, selected) in provinces_q.iter() {
                        if selected.is_some() {
                            commands.entity(entity).remove::<Selected>();
                        }
                    }

                    // Find the province entity with the matching ID
                    for (entity, province, _) in provinces_q.iter() {
                        if province.id == format!("PROV_{:03}", province_id) {
                            commands.entity(entity).insert(Selected);

                            if let Some(state) = states_q.iter().find(|s| s.id == province.state_id) {
                                let event = ToVueEvent::ProvinceSelected {
                                    province: province.clone(),
                                    state: state.clone(),
                                };
                                if let Ok(js_val) = serde_wasm_bindgen::to_value(&event) {
                                    send_to_js(js_val);
                                }
                            }
                            break;
                        }
                    }
                }
            }
        }
    }
}
