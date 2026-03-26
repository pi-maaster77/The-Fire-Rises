// devtools/map-editor/src/map/components.rs

use bevy::{
    prelude::*,
    utils::HashMap
};
use serde::Serialize;

// ======================================================
// 🗺️ MAP DOMAIN (Provinces / States / Regions)
// ======================================================

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

// ======================================================
// 🏷️ TAG COMPONENTS (markers, sin datos complejos)
// ======================================================

#[derive(Component)]
pub struct Selected;

#[allow(dead_code)]
#[derive(Component)]
pub struct StateBorder {
    pub state_id: u32,
}

#[derive(Component)]
pub struct MapSprite;

// ======================================================
// 🖼️ MAP RESOURCES (datos pesados del mapa)
// ======================================================

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

// ======================================================
// 🧠 EDITOR STATE (estado interno del editor)
// ======================================================

#[derive(Resource, Default)]
pub struct SelectedProvinceId(pub Option<String>);

#[derive(Resource, Default)]
pub struct ProvinceStateMap(pub HashMap<String, String>);

// ======================================================
// 🎥 CAMERA CONFIG
// ======================================================

#[derive(Resource)]
pub struct CameraConfig {
    pub move_speed: f32,
    pub zoom_speed: f32,
    pub pan_button: MouseButton,
}

impl Default for CameraConfig {
    fn default() -> Self {
        Self {
            move_speed: 500.0,
            zoom_speed: 0.1,
            pan_button: MouseButton::Right,
        }
    }
}