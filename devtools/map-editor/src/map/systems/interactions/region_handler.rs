// devtools/map-editor/src/map/systems/interactions/region_handler.rs

use bevy::prelude::*;
use crate::map::components::{Province, ActiveRegionEditing};
use crate::map::systems::interactions::events::ProvinceSelectedEvent;
use crate::bridge::outbound::send_to_vue;
use serde_json::json;

pub fn handle_region_assignment(
    mut events: EventReader<ProvinceSelectedEvent>,
    mut provinces: Query<&mut Province>,
    active_region: Res<ActiveRegionEditing>,
) {
    let Some(current_region_id) = &active_region.id else { return };

    for event in events.read() {
        // 1. Buscamos el state_id de la provincia clickeada
        let target_state_id = provinces.iter()
            .find(|p| p.id == event.province_id)
            .map(|p| p.state_id.clone());

        if let Some(state_id) = target_state_id {
            // 2. Buscamos TODAS las provincias que pertenecen a ese estado y les asignamos la región
            for mut province in provinces.iter_mut() {
                if province.state_id == state_id {
                    province.region_id = Some(current_region_id.clone());
                }
            }

            // 3. Notificamos a Vue que el estado completo cambió de región
            send_to_vue("STATE_ASSIGNED_TO_REGION", &json!({
                "state_id": state_id,
                "region_id": current_region_id
            }));
        }
    }
}