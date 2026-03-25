// frontend/map/src/map/systems.rs

use bevy::

{
	ecs::system::Commands, math::Vec2, prelude::*, render::color::Color, sprite::{Sprite, SpriteBundle}, transform::components::Transform
};
use serde::Serialize;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};

use crate::map::components::{Province, Region, Selected, State, StateBorder};

pub fn spawn_map(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // 1. Instanciamos las Regiones (Norte y Sur)
    commands.spawn(Region { id: 1, name: "Norte".into() });
    commands.spawn(Region { id: 2, name: "Sur".into() });

    // 2. Instanciamos los Estados
    // Guardamos los IDs que definimos en tu esquema
    let state_names = [
        (10, "Noreste", 1), (11, "Noroeste", 1),
        (20, "Sudeste", 2), (21, "Sudoeste", 2)
    ];

    for (id, name, reg_id) in state_names {
        commands.spawn(State { 
            id, 
            name: name.into(), 
            region_id: reg_id 
        });
    }

    // 3. Spawn de Provincias (Grilla 8x8)
    const TILE_SIZE: f32 = 50.0;
    for x in 0..8 {
        for y in 0..8 {
            let state_id = match (x < 4, y < 4) {
                (true, true) => 10, (false, true) => 11,
                (true, false) => 20, _ => 21,
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if state_id < 20 { Color::rgb(0.2, 0.4, 0.8) } else { Color::rgb(0.8, 0.2, 0.2) },
                        custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 - 3.5) * TILE_SIZE, // Ajuste para centrar 8 tiles
                        (y as f32 - 3.5) * TILE_SIZE,
                        0.0,
                    ),
                    ..default()
                },
                Province { id: (x * 10 + y) as u32, coords: (x, y), state_id },
            ));
        }
    }

    for x in 0..8 {
        for y in 0..8 {
            // INVERTIMOS LA LÓGICA DE ESTADO:
            // y >= 4 ahora será el Norte (Arriba)
            // y < 4 ahora será el Sur (Abajo)
            let state_id = match (x < 4, y >= 4) {
                (true, true) => 10,  // Noroeste (Arriba Izquierda)
                (false, true) => 11, // Noreste (Arriba Derecha)
                (true, false) => 20, // Sudoeste (Abajo Izquierda)
                _ => 21,             // Sudeste (Abajo Derecha)
            };

            commands.spawn((
                SpriteBundle {
                    sprite: Sprite {
                        color: if state_id < 20 { 
                            Color::rgb(0.2, 0.4, 0.8) 
                        } else { 
                            Color::rgb(0.8, 0.2, 0.2) 
                        },
                        custom_size: Some(Vec2::splat(TILE_SIZE - 2.0)),
                        ..default()
                    },
                    transform: Transform::from_xyz(
                        (x as f32 - 3.5) * TILE_SIZE,
                        (y as f32 - 3.5) * TILE_SIZE, // Bevy sube hacia arriba
                        0.0,
                    ),
                    ..default()
                },
                Province { id: (x * 10 + y) as u32, coords: (x, y), state_id },
            ));
        }
    }
}

// Definimos el evento que vamos a enviar a JS
#[derive(Serialize)]
#[serde(tag = "type", content = "payload")]
pub enum ToVueEvent {
    #[serde(rename = "PROVINCE_SELECTED")]
    ProvinceSelected { province: Province, state: State },
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = __BEVY_BRIDGE_INBOUND__)]
    fn send_to_js(event: JsValue);
}

pub fn handle_map_click(
    mut commands: Commands,
    mouse: Res<ButtonInput<MouseButton>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    window_q: Query<&Window>,
    // Añadimos Entity para poder poner/sacar componentes
    provinces_q: Query<(Entity, &Province, &GlobalTransform, &Sprite, Option<&Selected>)>,
    states_q: Query<&State>,
) {
    if mouse.just_pressed(MouseButton::Left) {
        let window = window_q.single();
        let (camera, camera_transform) = camera_q.single();

        if let Some(world_pos) = window.cursor_position()
            .and_then(|cursor| camera.viewport_to_world_2d(camera_transform, cursor)) 
        {
            // 1. Primero, limpiamos la selección previa
            for (entity, _, _, _, selected) in provinces_q.iter() {
                if selected.is_some() {
                    commands.entity(entity).remove::<Selected>();
                }
            }

            // 2. Buscamos la nueva provincia clickeada
            for (entity, prov, transform, sprite, _) in provinces_q.iter() {
                let size = sprite.custom_size.unwrap_or(Vec2::splat(50.0)) / 2.0;
                let pos = transform.translation().truncate();
                
                if world_pos.x > pos.x - size.x && world_pos.x < pos.x + size.x &&
                   world_pos.y > pos.y - size.y && world_pos.y < pos.y + size.y {
                    
                    // Marcamos como seleccionada en Rust
                    commands.entity(entity).insert(Selected);

                    // Avisamos a Vue (tu lógica actual)
                    if let Some(state) = states_q.iter().find(|s| s.id == prov.state_id) {
                        let event = ToVueEvent::ProvinceSelected { 
                            province: prov.clone(), 
                            state: state.clone() 
                        };
                        if let Ok(js_val) = serde_wasm_bindgen::to_value(&event) {
                            send_to_js(js_val);
                        }
                    }
                    break; 
                }
            }
        }
    }
}

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