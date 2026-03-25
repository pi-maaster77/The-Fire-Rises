// frontend/map/src/map/systems/setup.rs

use bevy::

{
	ecs::system::Commands, 
	math::Vec2, 
	prelude::*, 
	render::color::Color, 
	sprite::{Sprite, SpriteBundle}, 
	transform::components::Transform
};

use crate::map::components::{Province, Region, State};

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
