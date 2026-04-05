// devtools/map-editor/src/bridge/mod.rs

pub mod inbound;
pub mod outbound;
pub mod state;
pub mod systems;

use bevy::prelude::*;
use systems::assignments;

pub struct BridgePlugin;

impl Plugin for BridgePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (
            assignments::sync_brush_settings,
            assignments::sync_region_settings,
            assignments::create_region_system,
            assignments::create_state_system,
            assignments::trigger_export_event_system,
            assignments::export_map_system,
        ));
    }
}

