// devtools/map-editor/src/bridge/systems/load_image.rs

use bevy::ecs::system::{Commands, ResMut, Resource};

use crate::{bridge::state::MAP_IMAGE_DATA, map::components::{MapImage, ProvincePixelMap}};

#[derive(Resource)]
pub struct ScanTrigger;

pub fn check_load_image(
    mut commands: Commands,
    mut map_image: Option<ResMut<MapImage>>,
    mut pixel_map: Option<ResMut<ProvincePixelMap>>,
) {
    if let Some((data, width, height)) = MAP_IMAGE_DATA.lock().unwrap().take() {
        if let (Some(map_image), Some(pixel_map)) = (map_image.as_mut(), pixel_map.as_mut()) {
            map_image.width = width;
            map_image.height = height;
            map_image.data = data;
            pixel_map.width = width;
            pixel_map.height = height;
            pixel_map.data = vec![None; (width * height) as usize];
            commands.insert_resource(ScanTrigger);
        }
    }
}