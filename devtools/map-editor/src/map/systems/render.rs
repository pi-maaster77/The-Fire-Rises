// devtools/map-editor/src/map/systems/render.rs

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::render_asset::RenderAssetUsages;
use crate::bridge::systems::selection::RenderUpdateTrigger;
// Asegurate de importar SelectedProvinceId (el recurso que definimos antes)
use crate::map::components::{MapImage, MapSprite, ProvincePixelMap, SelectedProvinceId, ProvinceStateMap};


pub fn render_map_sprite(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    map_image: Res<MapImage>,
    pixel_map: Res<ProvincePixelMap>,
    selected_res: Res<SelectedProvinceId>, // Recurso con el ID actual
	state_map: Res<ProvinceStateMap>, // <--- FALTA ESTA LÍNEA
    query: Query<Entity, With<MapSprite>>,
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

					// COLOR POR DEFECTO (Gris tierra)
					let (mut r, mut g, mut b) = (60, 60, 60);

					// Si la provincia tiene un estado asignado, usamos su color hash
					if let Some(s_id) = state_map.0.get(prov_id) {
						let (sr, sg, sb) = get_state_color(s_id);
						r = sr; g = sg; b = sb;
					}

					if is_border {
						r = 0; g = 0; b = 0;
					}

					if is_selected && is_border {
						r = 255; g = 215; b = 0; // Dorado
					}

					data[idx * 4] = r;
					data[idx * 4 + 1] = g;
					data[idx * 4 + 2] = b;
					data[idx * 4 + 3] = 255;
				} else {
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

fn get_state_color(state_id: &str) -> (u8, u8, u8) {
    let mut hash: u32 = 0;
    for c in state_id.chars() {
        hash = (c as u32).wrapping_add(hash << 5).wrapping_sub(hash);
    }
    // Generamos un color RGB basado en el hash
    (
        (hash & 0xFF) as u8,
        ((hash >> 8) & 0xFF) as u8,
        ((hash >> 16) & 0xFF) as u8,
    )
}