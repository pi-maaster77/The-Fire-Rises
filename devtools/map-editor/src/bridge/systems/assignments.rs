// devtools/map-editor/src/bridge/systems/assignments.rs
use bevy::prelude::*;
use crate::{bridge::state::{BRUSH_UPDATE, ACTIVE_REGION_UPDATE, REGION_CREATION, STATE_CREATION, EXPORT_TRIGGER}, map::systems::interactions::brush::BrushSettings, map::components::{ActiveRegionEditing, Region, State, Province}, map::systems::interactions::events::ExportTriggerEvent};
use crate::bridge::outbound::send_to_vue;
use serde_json::json;

pub fn sync_brush_settings(mut brush_res: ResMut<BrushSettings>) {
    if let Ok(mut msg) = BRUSH_UPDATE.lock() {
        if let Some((active, state_id)) = msg.take() {
            brush_res.is_painting = active;
            brush_res.active_state_id = state_id;
        }
    }
}

// src/bridge/systems/assignments.rs
pub fn sync_region_settings(
    mut active_region: ResMut<ActiveRegionEditing>,
) {
    if let Ok(mut guard) = ACTIVE_REGION_UPDATE.lock() {
        if let Some(new_id) = guard.take() {
            active_region.id = new_id;
        }
    }
}

pub fn create_region_system(mut commands: Commands) {
    if let Ok(mut guard) = REGION_CREATION.lock() {
        if let Some((id, name)) = guard.take() {
            commands.spawn(Region { id, name });
        }
    }
}

pub fn create_state_system(mut commands: Commands) {
    if let Ok(mut guard) = STATE_CREATION.lock() {
        if let Some((id, name, region_id)) = guard.take() {
            commands.spawn(State { id, name, region_id });
        }
    }
}

pub fn export_map_system(
    mut events: EventReader<ExportTriggerEvent>,
    regions: Query<&Region>,
    states: Query<&State>,
    provinces: Query<&Province>,
) {
    if events.read().next().is_some() {
        // Collect regions data
        let regions_data: Vec<_> = regions.iter().map(|r| {
            json!({
                "id": r.id,
                "name": r.name
            })
        }).collect();

        // Collect states data
        let states_data: Vec<_> = states.iter().map(|s| {
            json!({
                "id": s.id,
                "name": s.name,
                "region_id": s.region_id
            })
        }).collect();

        // Collect provinces data with seed points
        let provinces_data: Vec<_> = provinces.iter().map(|p| {
            json!({
                "id": p.id.clone(),
                "state_id": p.state_id.clone(),
                "region_id": p.region_id.clone(),
                "center": [p.center.x, p.center.y]
            })
        }).collect();

        // For now, we'll use empty seed_points since we need to get them from the map generation
        // In a full implementation, this would come from the Voronoi diagram data
        let seed_points: Vec<[f32; 2]> = vec![]; // TODO: Get from map generation system

        let export_data = json!({
            "version": "1.0",
            "map_params": {
                "width": 800,  // TODO: Get from actual map params
                "height": 600,
                "voronoi_points": 35,
                "lloyd_iterations": 2
            },
            "seed_points": seed_points,
            "regions": regions_data,
            "states": states_data,
            "provinces": provinces_data
        });

        // Send to Vue
        send_to_vue("MAP_EXPORTED", &export_data);
    }
}

pub fn trigger_export_event_system(mut events: EventWriter<ExportTriggerEvent>) {
    if let Ok(mut guard) = EXPORT_TRIGGER.lock() {
        if *guard {
            *guard = false;
            events.send(ExportTriggerEvent);
        }
    }
}