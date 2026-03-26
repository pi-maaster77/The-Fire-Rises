// devtools/map-editor/src/bridge/systems/assignments.rs

// devtools/map-editor/src/bridge/systems/assignments.rs
use bevy::prelude::*;
use crate::{bridge::state::BRUSH_UPDATE, map::systems::interactions::brush::BrushSettings};

pub fn sync_brush_settings(mut brush_res: ResMut<BrushSettings>) {
    if let Ok(mut msg) = BRUSH_UPDATE.lock() {
        if let Some((active, state_id)) = msg.take() {
            brush_res.is_painting = active;
            brush_res.active_state_id = state_id;
        }
    }
}

// src/bridge/systems/assignments.rs
use crate::bridge::state::ACTIVE_REGION_UPDATE;
use crate::map::components::ActiveRegionEditing;

pub fn sync_region_settings(
    mut active_region: ResMut<ActiveRegionEditing>,
) {
    if let Ok(mut guard) = ACTIVE_REGION_UPDATE.lock() {
        if let Some(new_id) = guard.take() {
            active_region.id = new_id;
        }
    }
}