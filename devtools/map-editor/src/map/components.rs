// frontend/map/src/map/components.rs

use bevy::prelude::*;
use serde::Serialize;

#[derive(Component, Serialize, Clone)]
pub struct Province {
    pub id: u32,
    pub coords: (i32, i32),
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