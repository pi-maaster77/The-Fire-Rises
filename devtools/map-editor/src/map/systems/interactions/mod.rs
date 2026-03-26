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
    {    // IMPORTANTE: En la Web/Bevy, cursor_pos.y = 0 es ARRIBA.
        // Pero en nuestra matriz de píxeles cargada desde JS, el índice 0 suele ser ARRIBA.
        // Si notas que el click está invertido, ajustá esta línea:
        let x = (world_pos.x + (pixel_map.width as f32 / 2.0)) as u32;
        // Invertimos el eje Y porque Bevy sube y las imágenes bajan.
        let y = ((pixel_map.height as f32 / 2.0) - world_pos.y) as u32;

        if x < pixel_map.width && y < pixel_map.height {
            let idx = (y * pixel_map.width + x) as usize;
            
            if let Some(prov_id) = pixel_map.data.get(idx).and_then(|id| id.as_ref()) {
                // 1. Actualizar el Recurso de resaltado
                selected_res.0 = Some(prov_id.clone());
                
                // 2. Disparar el repintado
                commands.insert_resource(RenderUpdateTrigger);

                // 3. Manejar la entidad y enviar a Vue
                for (entity, province) in provinces.iter() {
                    if &province.id == prov_id {
                        // Limpiar selección previa de entidades
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




// ... actualizar la firma de la función ...
pub fn handle_click2(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    // Necesitamos la cámara para la proyección
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse: Res<ButtonInput<MouseButton>>,
    pixel_map: Res<ProvincePixelMap>,
    mut selected_res: ResMut<SelectedProvinceId>,
    provinces: Query<(Entity, &Province)>,
    selected_entities: Query<Entity, With<Selected>>,
) {
    if !mouse.just_pressed(MouseButton::Left) { return; }
    
    let Ok(window) = window_query.get_single() else { return };
    // Obtenemos la cámara 2D
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return };
    
    // 1. Obtenemos la posición del cursor en el mundo (proyectada por la cámara)
    if let Some(world_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        // LA CLAVE DE LA CORRECCIÓN:
        // world_pos ahora es Vec2(x, y) donde 0,0 suele ser el centro de la pantalla.
        // Pero nuestro PixelMap empieza en 0,0 (arriba izquierda).

        // Como tu mapa es de 800x600 y el Sprite está en 0,0, el mundo va de:
        // x: -400 a 400
        // y: -300 a 300

        // 2. Traducimos la posición del mundo a índice de matriz (0 a 800, 0 a 600)
        // Sumamos la mitad del ancho/alto para des-centrarlo.
        let x = (world_pos.x + (pixel_map.width as f32 / 2.0)) as u32;
        // Invertimos el eje Y porque Bevy sube y las imágenes bajan.
        let y = ((pixel_map.height as f32 / 2.0) - world_pos.y) as u32;

        // 3. Chequeamos límites antes de acceder
        if x < pixel_map.width && y < pixel_map.height {
            let idx = (y * pixel_map.width + x) as usize;
            
            if let Some(prov_id) = pixel_map.data.get(idx).and_then(|id| id.as_ref()) {
                // ... resto de tu lógica para seleccionar y enviar a Vue ...
                
                // Trigger de repintado para que se vea el borde dorado
                selected_res.0 = Some(prov_id.clone());
                commands.insert_resource(RenderUpdateTrigger);
            }
        }
    }
}