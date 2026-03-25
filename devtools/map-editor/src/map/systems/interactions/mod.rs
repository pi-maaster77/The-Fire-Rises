// devtools/map-editor/src/map/systems/interactions/mod.rs

use bevy::prelude::*;
use bevy::input::mouse::MouseButton;
use bevy::window::PrimaryWindow;
use crate::map::components::{Province, ProvincePixelMap, Selected};
use crate::bridge::send_to_vue;
use serde_json::json;

pub fn handle_click(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mouse: Res<ButtonInput<MouseButton>>,
    pixel_map: Res<ProvincePixelMap>,
    provinces: Query<(Entity, &Province)>,
    selected: Query<Entity, With<Selected>>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = window_query.single();
        if let Some(cursor_pos) = window.cursor_position() {
            // Assume image is displayed at 0,0 to 800,600
            let x = (cursor_pos.x as u32).min(pixel_map.width.saturating_sub(1));
            let y = (cursor_pos.y as u32).min(pixel_map.height.saturating_sub(1));
            let idx = (y * pixel_map.width + x) as usize;
            if let Some(prov_id) = &pixel_map.data[idx] {
                // Find the province entity
                for (entity, province) in provinces.iter() {
                    if &province.id == prov_id {
                        // Deselect previous
                        for sel in selected.iter() {
                            commands.entity(sel).remove::<Selected>();
                        }
                        // Select this
                        commands.entity(entity).insert(Selected);
                        // Send to Vue
                        let payload = json!({
                            "province": province,
                            "state": {"id": province.state_id, "name": "Placeholder"}
                        });
                        send_to_vue("PROVINCE_SELECTED", &payload);
                        break;
                    }
                }
            }
        }
    }
}
