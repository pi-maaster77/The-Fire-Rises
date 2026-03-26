// devtools/map-editor/src/map/systems/interactions/mod.rs

use bevy::prelude::*;

pub mod events;
pub mod brush;
mod click_handler; // Podés separar la lógica de click aquí

use crate::map::systems::interactions::brush::brush_system;
use crate::map::systems::interactions::brush::BrushSettings;
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
								brush_system
            ).chain());
            
        // El pincel podría registrar sus propios sistemas aquí también
    }
}