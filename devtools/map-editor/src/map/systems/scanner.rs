// devtools/map-editor/src/map/systems/scanner.rs

use bevy::prelude::*;
use crate::map::components::{Province, MapImage, ProvincePixelMap};
use crate::bridge::{ScanTrigger, RenderUpdateTrigger, send_to_vue};
use serde_json::json;

pub fn scanner_system(
    mut commands: Commands,
    mut scan_trigger: ResMut<ScanTrigger>,
    map_image: Res<MapImage>,
    mut pixel_map: ResMut<ProvincePixelMap>,
) {
    // Remove the trigger
    commands.remove_resource::<ScanTrigger>();

    let mut visited = vec![false; (map_image.width * map_image.height) as usize];
    let mut province_counter = 0;
    let mut province_ids = Vec::new();

    for y in 0..map_image.height {
        for x in 0..map_image.width {
            let idx = (y * map_image.width + x) as usize;
            if visited[idx] {
                continue;
            }
            let pixel = get_pixel(&map_image.data, idx);
            if pixel[3] == 0 { // transparent
                continue;
            }
            // Start flood fill
            let mut pixels = Vec::new();
            flood_fill(x, y, pixel, &map_image, &mut visited, &mut pixels);
            if pixels.is_empty() {
                continue;
            }
            // Calculate centroid
            let mut sum_x = 0.0;
            let mut sum_y = 0.0;
            for (px, py) in &pixels {
                sum_x += *px as f32;
                sum_y += *py as f32;
                let pidx = (*py * map_image.width + *px) as usize;
                pixel_map.data[pidx] = Some(format!("PROV_{:03}", province_counter + 1));
            }
            let center = Vec2::new(sum_x / pixels.len() as f32, sum_y / pixels.len() as f32);
            // Spawn province
            let id = format!("PROV_{:03}", province_counter + 1);
            commands.spawn(Province {
                id: id.clone(),
                center,
                state_id: "STATE_001".to_string(), // placeholder
            });
            province_ids.push(id);
            province_counter += 1;
        }
    }

    // Send to Vue
    let payload = json!({ "provinces": province_ids });
    send_to_vue("SCAN_COMPLETED", &payload);

    // Trigger render update
    commands.insert_resource(RenderUpdateTrigger);
}

fn get_pixel(data: &[u8], idx: usize) -> [u8; 4] {
    [
        data[idx * 4],
        data[idx * 4 + 1],
        data[idx * 4 + 2],
        data[idx * 4 + 3],
    ]
}

fn flood_fill(
    start_x: u32,
    start_y: u32,
    color: [u8; 4],
    map_image: &MapImage,
    visited: &mut [bool],
    pixels: &mut Vec<(u32, u32)>,
) {
    let mut stack = vec![(start_x, start_y)];
    while let Some((x, y)) = stack.pop() {
        let idx = (y * map_image.width + x) as usize;
        if visited[idx] {
            continue;
        }
        visited[idx] = true;
        let pixel = get_pixel(&map_image.data, idx);
        if pixel != color {
            continue;
        }
        pixels.push((x, y));
        // Add neighbors
        if x > 0 {
            stack.push((x - 1, y));
        }
        if x < map_image.width - 1 {
            stack.push((x + 1, y));
        }
        if y > 0 {
            stack.push((x, y - 1));
        }
        if y < map_image.height - 1 {
            stack.push((x, y + 1));
        }
    }
}