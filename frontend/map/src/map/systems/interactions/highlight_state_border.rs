// frontend/map/src/map/systems/interactions/highlight_state_border.rs


use bevy::

{
	prelude::*, 
	render::color::Color, 
	sprite::{Sprite}
};


use crate::map::components::{Province, Selected, StateBorder};

pub fn highlight_state_border(
    selected_province_q: Query<&Province, With<Selected>>,
    mut borders_q: Query<(&mut Sprite, &StateBorder)>,
) {
    // 1. Obtenemos el ID del estado seleccionado (si hay uno)
    let active_state_id = selected_province_q.get_single().ok().map(|p| p.state_id);

    // 2. Actualizamos todos los bordes
    for (mut sprite, border) in borders_q.iter_mut() {
        if Some(border.state_id) == active_state_id {
            // Si es el estado seleccionado, mostramos un borde blanco grueso
            sprite.color = Color::rgb(1.0, 1.0, 1.0); 
            // Podés jugar con el grosor usando un mesh o simplemente un sprite con borde
        } else {
            // Si no, lo mantenemos invisible o con un color muy tenue
            sprite.color = Color::NONE;
        }
    }
}