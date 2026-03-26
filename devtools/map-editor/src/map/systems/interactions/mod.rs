// devtools/map-editor/src/map/systems/interactions/mod.rs

use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::map::components::{Province, ProvincePixelMap, Selected, SelectedProvinceId};
use crate::bridge::{send_to_vue, RenderUpdateTrigger};
use serde_json::json;
use bevy::render::camera::Camera;
use bevy::transform::components::GlobalTransform;

pub fn handle_click(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse: Res<ButtonInput<MouseButton>>,
    pixel_map: Res<ProvincePixelMap>,
    mut selected_res: ResMut<SelectedProvinceId>,
    provinces: Query<(Entity, &Province)>,
    selected_entities: Query<Entity, With<Selected>>,
) {
    if !mouse.just_pressed(MouseButton::Left) { return; }
    
    let Ok(window) = window_query.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return };

    if let Some(world_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {    
        // 1. Convertimos World Space a Image Space
        // El sprite está en 0,0 (centro). La imagen mide pixel_map.width x height.
        let half_w = pixel_map.width as f32 / 2.0;
        let half_h = pixel_map.height as f32 / 2.0;

        let x_f = world_pos.x + half_w;
        let y_f = half_h - world_pos.y; // Invertir Y porque en Bevy +Y es arriba

        // 2. Verificamos límites
        if x_f >= 0.0 && x_f < pixel_map.width as f32 && 
           y_f >= 0.0 && y_f < pixel_map.height as f32 {
            
            let x = x_f as u32;
            let y = y_f as u32;
            let idx = (y * pixel_map.width + x) as usize;
            
            if let Some(prov_id) = pixel_map.data.get(idx).and_then(|id| id.as_ref()) {
                selected_res.0 = Some(prov_id.clone());
                commands.insert_resource(RenderUpdateTrigger);

                for (entity, province) in provinces.iter() {
                    if &province.id == prov_id {
                        for sel in selected_entities.iter() {
                            commands.entity(sel).remove::<Selected>();
                        }
                        commands.entity(entity).insert(Selected);

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