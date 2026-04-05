// devtools/map-editor/src/map/systems/interactions/events.rs

use bevy::prelude::*;

#[derive(Event)]
pub struct MapClickEvent {
    pub world_pos: Vec2,
}

#[derive(Event)]
pub struct ProvinceSelectedEvent {
    pub province_id: String,
}

#[derive(Event)]
pub struct ExportTriggerEvent;

