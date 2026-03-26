// devtools/map-editor/src/bridge/systems/assignments.rs

use bevy::ecs::system::{Commands, Query};

use crate::{bridge::{state::STATE_ASSIGNMENTS, systems::selection::RenderUpdateTrigger}, map::components::Province};

pub fn sync_state_assignments(
    mut commands: Commands,
    mut provinces: Query<&mut Province>,
) {
    if let Ok(mut assignments) = STATE_ASSIGNMENTS.lock() {
        for (prov_id, state_id) in assignments.drain(..) {
            for mut province in provinces.iter_mut() {
                if province.id == prov_id {
                    province.state_id = state_id.clone();
                    // Trigger para repintar el mapa
                    commands.insert_resource(RenderUpdateTrigger);
                }
            }
        }
    }
}