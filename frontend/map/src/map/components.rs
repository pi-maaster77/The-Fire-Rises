// frontend/map/src/map/components.rs

use bevy::prelude::*;
use serde::Serialize;

#[derive(Component, Serialize, Clone)]
pub struct Province {
    pub id: String,
    pub center: Vec2,
    pub state_id: u32,
}

#[derive(Component, Serialize, Clone)]
pub struct State {
    pub id: u32,
    pub name: String,
    pub region_id: u32,
}

#[derive(Component, Serialize, Clone)]
pub struct Region {
    pub id: u32,
    pub name: String,
}

#[derive(Component)]
pub struct Selected;

#[derive(Component)]
pub struct StateBorder {
    pub state_id: u32,
}

#[derive(Resource)]
pub struct ProvinceMap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<Option<u32>>,
}

impl ProvinceMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![None; (width * height) as usize],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> Option<u32> {
        self.data[(y * self.width + x) as usize]
    }

    pub fn set(&mut self, x: u32, y: u32, value: u32) {
        self.data[(y * self.width + x) as usize] = Some(value);
    }
}