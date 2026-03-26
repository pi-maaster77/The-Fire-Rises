// devtools/map-editor/src/map/systems/interactions/brush.rs

use bevy::core_pipeline::core_2d::Camera2d;
use bevy::ecs::query::With;
use bevy::ecs::system::{Commands, Query, Res, ResMut};
use bevy::input::ButtonInput;
use bevy::input::mouse::MouseButton;
use bevy::render::camera::Camera;
use bevy::transform::components::GlobalTransform;
use bevy::window::{PrimaryWindow, Window};

use crate::bridge::systems::selection::RenderUpdateTrigger;
use crate::map::components::{BrushSettings, ProvinceStateMap, ProvincePixelMap};

pub fn brush_system(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform), With<Camera2d>>,
    mouse: Res<ButtonInput<MouseButton>>,
    brush: Res<BrushSettings>,
    pixel_map: Res<ProvincePixelMap>,
    mut state_map: ResMut<ProvinceStateMap>,
) {
    // Solo actuamos si el modo pincel está activo y el click izquierdo está presionado
    if !brush.is_painting || !mouse.pressed(MouseButton::Left) {
        return;
    }

    let Some(target_state) = &brush.active_state_id else { return };

    let Ok(window) = window_query.get_single() else { return };
    let Ok((camera, camera_transform)) = camera_q.get_single() else { return };

    if let Some(world_pos) = window.cursor_position()
        .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor))
    {
        let half_w = pixel_map.width as f32 / 2.0;
        let half_h = pixel_map.height as f32 / 2.0;

        let x = (world_pos.x + half_w) as u32;
        let y = (half_h - world_pos.y) as u32;

        if x < pixel_map.width && y < pixel_map.height {
            let idx = (y * pixel_map.width + x) as usize;
            if let Some(prov_id) = &pixel_map.data[idx] {
                
                // Solo disparamos render si realmente cambió algo
                let current_state = state_map.0.get(prov_id);
                if current_state != Some(target_state) {
                    state_map.0.insert(prov_id.clone(), target_state.clone());
                    commands.insert_resource(RenderUpdateTrigger);
                }
            }
        }
    }
}