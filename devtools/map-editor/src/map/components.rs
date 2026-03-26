// devtools/map-editor/src/map/components.rs

use bevy::{
	prelude::*,
	utils::HashMap
};
use serde::Serialize;

#[derive(Component, Serialize, Clone)]
pub struct Province {
    pub id: String,
    pub center: Vec2,
    pub state_id: String,
}

#[allow(dead_code)]
#[derive(Component, Serialize, Clone)]
pub struct State {
    pub id: String,
    pub name: String,
    pub region_id: String,
}

#[allow(dead_code)]
#[derive(Component, Serialize, Clone)]
pub struct Region {
    pub id: String,
    pub name: String,
}

#[derive(Component)]
pub struct Selected;

#[allow(dead_code)]
#[derive(Component)]
pub struct StateBorder {
    pub state_id: u32,
}

#[derive(Resource)]
pub struct MapImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>, // RGBA
}

#[derive(Resource)]
pub struct ProvincePixelMap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Option<String>>, // province id per pixel
}

#[derive(Component)]
pub struct MapSprite;

// map/components.rs
#[derive(Resource, Default)]
pub struct SelectedProvinceId(pub Option<String>);

#[derive(Resource, Default)]
pub struct ProvinceStateMap(pub HashMap<String, String>);

#[derive(Resource, Default)]
pub struct BrushSettings {
    pub active_state_id: Option<String>,
    pub is_painting: bool,
}