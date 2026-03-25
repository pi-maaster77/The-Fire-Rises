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
}

pub fn handle_map_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    window_q: Query<&Window>,
    // Añadimos Entity para poder poner/sacar componentes
    provinces_q: Query<(Entity, &Province, &GlobalTransform, &Sprite, Option<&Selected>)>,
    states_q: Query<&State>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = window_q.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(world_pos) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) 
        {
            // 1. Primero, limpiamos la selección previa
            for (entity, _, _, _, selected) in provinces_q.iter() {
                if selected.is_some() {
                    commands.entity(entity).remove::<Selected>();
                }
            }

            // 2. Buscamos la nueva provincia clickeada
            for (entity, prov, transform, sprite, _) in provinces_q.iter() {
                let size = sprite.custom_size.unwrap_or(Vec2::splat(50.0)) / 2.0;
                let pos = transform.translation().truncate();
                
                if world_pos.x > pos.x - size.x && world_pos.x < pos.x + size.x &&
                   world_pos.y > pos.y - size.y && world_pos.y < pos.y + size.y {
                    
                    // Marcamos como seleccionada en Rust
                    commands.entity(entity).insert(Selected);

                    // Avisamos a Vue (tu lógica actual)
                    if let Some(state) = states_q.iter().find(|s| s.id == prov.state_id) {
                        let event = ToVueEvent::ProvinceSelected { 
                            province: prov.clone(), 
                            state: state.clone() 
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
