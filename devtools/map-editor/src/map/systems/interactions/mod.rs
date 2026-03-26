// devtools/map-editor/src/map/systems/interactions/mod.rs

use bevy::prelude::*;

pub mod events;
pub mod brush;
pub mod region_handler;
mod click_handler; // Podés separar la lógica de click aquí

use crate::map::systems::interactions::brush::brush_system;
use crate::map::systems::interactions::brush::BrushSettings;
use self::region_handler::handle_region_assignment;
use self::events::{MapClickEvent, ProvinceSelectedEvent};
use self::click_handler::{emit_map_events, resolve_province_click, handle_province_selection};

pub struct InteractionsPlugin;

impl Plugin for InteractionsPlugin {
    fn build(&self, app: &mut App) {
        app
				    .init_resource::<BrushSettings>()
            .add_event::<MapClickEvent>()
            .add_event::<ProvinceSelectedEvent>()
            .add_systems(Update, (
                emit_map_events,
                resolve_province_click,
                handle_province_selection,
								brush_system,
								handle_region_assignment,
            ).chain());
            
        // El pincel podría registrar sus propios sistemas aquí también
    }
}