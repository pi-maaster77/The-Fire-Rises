// devtools/map-editor/src/bridge/systems/selection.rs

use bevy::ecs::system::{Commands, ResMut, Resource};

use crate::{bridge::state::EXTERNAL_SELECTION, map::components::SelectedProvinceId};

#[derive(Resource)]
pub struct RenderUpdateTrigger;



pub fn check_external_selection(
    mut commands: Commands,
    mut selected_res: ResMut<SelectedProvinceId>,
) {
    if let Ok(mut guard) = EXTERNAL_SELECTION.lock() {
        if let Some(id) = guard.take() {
            // Actualizamos el recurso y disparamos el repintado (el borde dorado)
            selected_res.0 = Some(id);
            commands.insert_resource(RenderUpdateTrigger);
        }
    }
}
