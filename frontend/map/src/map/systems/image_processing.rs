use bevy::prelude::*;
use crate::map::components::{Province, ProvinceMap};
use std::collections::VecDeque;
use wasm_bindgen::{JsValue, prelude::wasm_bindgen};
use crate::map::systems::interactions::handle_map_click::ToVueEvent;

#[wasm_bindgen]
extern "C" {
    // Un solo punto de entrada en JS para todos los eventos
    #[wasm_bindgen(js_name = __BEVY_BRIDGE_INBOUND__)]
    fn send_to_js(event: JsValue);
}

pub fn process_map_image(
    mut commands: Commands,
    mut ev_asset: EventReader<AssetEvent<Image>>,
    assets: Res<Assets<Image>>,
    mut province_map: ResMut<ProvinceMap>,
) {
    for ev in ev_asset.read() {
        if let AssetEvent::LoadedWithDependencies { id } = ev {
            let image = assets.get(*id).unwrap();
            let (width, height) = (image.texture_descriptor.size.width, image.texture_descriptor.size.height);

            let mut visited = vec![vec![false; height as usize]; width as usize];
            let mut province_id_counter = 0;

            for y in 0..height {
                for x in 0..width {
                    if visited[x as usize][y as usize] {
                        continue;
                    }

                    let color = get_pixel_color(image, x, y);

                    if color[3] == 0 { // Skip transparent pixels
                        continue;
                    }

                    let mut pixels_to_visit = VecDeque::new();
                    pixels_to_visit.push_back((x, y));
                    visited[x as usize][y as usize] = true;

                    let mut province_pixels = Vec::new();
                    let mut sum_x = 0;
                    let mut sum_y = 0;

                    while let Some((px, py)) = pixels_to_visit.pop_front() {
                        province_pixels.push((px, py));
                        sum_x += px;
                        sum_y += py;
                        province_map.set(px, py, province_id_counter);

                        for (nx, ny) in get_neighbors(px, py, width, height) {
                            if !visited[nx as usize][ny as usize] && get_pixel_color(image, nx, ny) == color {
                                visited[nx as usize][ny as usize] = true;
                                pixels_to_visit.push_back((nx, ny));
                            }
                        }
                    }

                    if !province_pixels.is_empty() {
                        let center_x = sum_x as f32 / province_pixels.len() as f32;
                        let center_y = sum_y as f32 / province_pixels.len() as f32;

                        let province_id = format!("PROV_{:03}", province_id_counter);

                        commands.spawn(Province {
                            id: province_id,
                            center: Vec2::new(center_x, center_y),
                            state_id: 0, // Placeholder
                        });
                        province_id_counter += 1;
                    }
                }
            }
            let provinces = (0..province_id_counter).map(|i| format!("PROV_{:03}", i)).collect();
            let event = ToVueEvent::ProvincesList { provinces };
            if let Ok(js_val) = serde_wasm_bindgen::to_value(&event) {
                send_to_js(js_val);
            }
        }
    }
}

fn get_pixel_color(image: &Image, x: u32, y: u32) -> [u8; 4] {
    let width = image.texture_descriptor.size.width as usize;
    let index = (y as usize * width + x as usize) * 4;
    [
        image.data[index],
        image.data[index + 1],
        image.data[index + 2],
        image.data[index + 3],
    ]
}

fn get_neighbors(x: u32, y: u32, width: u32, height: u32) -> Vec<(u32, u32)> {
    let mut neighbors = Vec::new();
    if x > 0 { neighbors.push((x - 1, y)); }
    if x < width - 1 { neighbors.push((x + 1, y)); }
    if y > 0 { neighbors.push((x, y - 1)); }
    if y < height - 1 { neighbors.push((x, y + 1)); }
    neighbors
}
