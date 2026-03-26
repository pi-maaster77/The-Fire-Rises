// devtools/map-editor/src/map/systems/render.rs

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::render_asset::RenderAssetUsages;
// Asegurate de importar SelectedProvinceId (el recurso que definimos antes)
use crate::map::components::{MapImage, MapSprite, ProvincePixelMap, SelectedProvinceId};
use crate::bridge::RenderUpdateTrigger;

pub fn render_map_sprite(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    map_image: Res<MapImage>,
    pixel_map: Res<ProvincePixelMap>,
    selected_res: Res<SelectedProvinceId>, // Recurso con el ID actual
    mut query: Query<Entity, With<MapSprite>>,
    render_trigger: Option<Res<RenderUpdateTrigger>>,
) {
    // Si no hay datos de mapa todavía, no hacemos nada
    if map_image.data.is_empty() || pixel_map.data.is_empty() {
        return;
    }

    let should_render = if render_trigger.is_some() {
        commands.remove_resource::<RenderUpdateTrigger>();
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        true
    } else {
        query.is_empty()
    };

    if should_render {
        let mut data = vec![0u8; (pixel_map.width * pixel_map.height * 4) as usize];

        for y in 0..pixel_map.height {
            for x in 0..pixel_map.width {
                let idx = (y * pixel_map.width + x) as usize;
								if let Some(prov_id) = &pixel_map.data[idx] {
										let is_selected = selected_res.0.as_ref() == Some(prov_id);
										let is_border = is_border_pixel(x, y, &pixel_map);

										// 1. Definimos los colores base (Verde para el interior, Negro para el borde)
										let mut r = 70;  
										let mut g = 180;
										let mut b = 70;

										if is_border {
												r = 0; g = 0; b = 0;
										}

										// 2. LA CORRECCIÓN: Si está seleccionada Y es borde, sobreescribimos a dorado.
										if is_selected && is_border {
												r = 255; g = 215; b = 0; // Dorado intenso
										}

										data[idx * 4] = r;
										data[idx * 4 + 1] = g;
										data[idx * 4 + 2] = b;
										data[idx * 4 + 3] = 255;
								} else {
											// Fondo transparente o vacío
											data[idx * 4 + 3] = 0; 
                }
            }
        }
				

        let image = Image::new(
            Extent3d { width: pixel_map.width, height: pixel_map.height, depth_or_array_layers: 1 },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::MAIN_WORLD | RenderAssetUsages::RENDER_WORLD,
        );

        let texture_handle = images.add(image);
        commands.spawn((
            SpriteBundle {
                texture: texture_handle,
                transform: Transform::from_xyz(0.0, 0.0, 0.0),
                ..default()
            },
            MapSprite,
        ));
    }
}

fn is_border_pixel(x: u32, y: u32, pixel_map: &ProvincePixelMap) -> bool {
    let current = &pixel_map.data[(y * pixel_map.width + x) as usize];
    let directions = [(0i32, -1i32), (0, 1), (-1, 0), (1, 0)];

    for (dx, dy) in directions {
        let nx = x as i32 + dx;
        let ny = y as i32 + dy;
        if nx >= 0 && nx < pixel_map.width as i32 && ny >= 0 && ny < pixel_map.height as i32 {
            let nidx = (ny as u32 * pixel_map.width + nx as u32) as usize;
            let neighbor = &pixel_map.data[nidx];
            if neighbor != current {
                return true;
            }
        } else {
            // Edge of map is border
            return true;
        }
    }
    false
}