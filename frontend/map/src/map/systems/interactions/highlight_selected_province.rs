// frontend/map/src/map/systems/interactions/highlight_selected_province.rs
use bevy::

{
	prelude::*, 
	render::color::Color, 
	sprite::{Sprite}
};

use crate::map::components::{Province, Selected};

pub fn highlight_selected_province(
    mut query: Query<(&mut Sprite, &Province, Option<&Selected>)>,
) {
    for (mut sprite, prov, selected) in query.iter_mut() {
        if selected.is_some() {
            // Blanco brillante o amarillo para la seleccionada
            sprite.color = Color::rgb(1.0, 1.0, 0.5); 
        } else {
            // Color original según el estado
            sprite.color = if prov.state_id < 20 { 
                Color::rgb(0.2, 0.4, 0.8) 
            } else { 
                Color::rgb(0.8, 0.2, 0.2) 
            };
        }
    }
}

