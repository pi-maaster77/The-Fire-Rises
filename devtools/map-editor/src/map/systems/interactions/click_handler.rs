// devtools/map-editor/src/map/systems/interactions/click_handler.rs

use bevy::{prelude::*, window::PrimaryWindow};
use serde_json::json;

use crate::{bridge::{outbound::send_to_vue, systems::selection::RenderUpdateTrigger}, map::{components::{Province, ProvincePixelMap, Selected, SelectedProvinceId}, systems::interactions::events::{MapClickEvent, ProvinceSelectedEvent}}};
// ... (resto de imports de componentes y bridge)

// map/systems/interactions/mod.rs
pub fn emit_map_events(
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse: Res<ButtonInput<MouseButton>>,
    mut event_writer: EventWriter<MapClickEvent>,
) {
    if !mouse.just_pressed(MouseButton::Left) { return; }
    
    let Ok(window) = window_query.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return };

    if let Some(world_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) 
    {
        event_writer.send(MapClickEvent { world_pos });
    }
}

pub fn resolve_province_click(
    mut events: EventReader<MapClickEvent>,
    mut selection_writer: EventWriter<ProvinceSelectedEvent>,
    pixel_map: Res<ProvincePixelMap>,
) {
    for event in events.read() {
        let half_w = pixel_map.width as f32 / 2.0;
        let half_h = pixel_map.height as f32 / 2.0;
        let x_f = event.world_pos.x + half_w;
        let y_f = half_h - event.world_pos.y;

        if x_f >= 0.0 && x_f < pixel_map.width as f32 && y_f >= 0.0 && y_f < pixel_map.height as f32 {
            let idx = (y_f as u32 * pixel_map.width + x_f as u32) as usize;
            if let Some(Some(prov_id)) = pixel_map.data.get(idx) {
                selection_writer.send(ProvinceSelectedEvent { province_id: prov_id.clone() });
            }
        }
    }
}

pub fn handle_province_selection(
    mut commands: Commands,
    mut events: EventReader<ProvinceSelectedEvent>,
    mut selected_res: ResMut<SelectedProvinceId>,
    provinces: Query<(Entity, &Province)>,
    selected_entities: Query<Entity, With<Selected>>,
) {
    for event in events.read() {
        selected_res.0 = Some(event.province_id.clone());
        commands.insert_resource(RenderUpdateTrigger);

        // Limpiar selección vieja e insertar nueva
        for (entity, province) in provinces.iter() {
            if province.id == event.province_id {
                for sel in selected_entities.iter() {
                    commands.entity(sel).remove::<Selected>();
                }
                commands.entity(entity).insert(Selected);

                // Notificar a Vue (podes mover esto a un sistema aparte incluso!)
                let payload = json!({ "province": province });
                send_to_vue("PROVINCE_SELECTED", &payload);
                break;
            }
        }
    }
}