// devtools/map-editor/src/map/systems/render.rs

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use bevy::render::render_asset::RenderAssetUsages;
use crate::map::components::{MapImage, MapSprite, ProvincePixelMap};
use crate::bridge::RenderUpdateTrigger;

pub fn render_map_sprite(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    map_image: Res<MapImage>,
    pixel_map: Res<ProvincePixelMap>,
    mut query: Query<Entity, With<MapSprite>>,
    render_trigger: Option<Res<RenderUpdateTrigger>>,
) {
    let should_render = if let Some(_) = render_trigger {
        // Remove trigger
        commands.remove_resource::<RenderUpdateTrigger>();
        // Despawn existing sprite
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }
        true
    } else {
        // Initial render when pixel_map is populated
        !map_image.data.is_empty() && !pixel_map.data.is_empty() && query.is_empty()
    };

    if should_render {
        // Generate synthetic image: green background, black borders for provinces
        let mut data = vec![0u8; (pixel_map.width * pixel_map.height * 4) as usize];

        // Initialize to green
        for i in 0..data.len() / 4 {
            data[i * 4] = 0;     // R
            data[i * 4 + 1] = 255; // G
            data[i * 4 + 2] = 0;   // B
            data[i * 4 + 3] = 255; // A
        }

        // Draw black borders where provinces are
        for y in 0..pixel_map.height {
            for x in 0..pixel_map.width {
                let idx = (y * pixel_map.width + x) as usize;
                if pixel_map.data[idx].is_some() {
                    // Check if this is a border pixel
                    let is_border = is_border_pixel(x, y, &pixel_map);
                    if is_border {
                        data[idx * 4] = 0;     // R
                        data[idx * 4 + 1] = 0; // G
                        data[idx * 4 + 2] = 0; // B
                        data[idx * 4 + 3] = 255; // A
                    }
                }
            }
        }

        // Create Bevy Image
        let image = Image::new(
            Extent3d {
                width: pixel_map.width,
                height: pixel_map.height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::RENDER_WORLD,
        );

        let texture_handle = images.add(image);

        // Spawn sprite
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