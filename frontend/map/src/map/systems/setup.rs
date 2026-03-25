// frontend/map/src/map/systems/setup.rs

use bevy::prelude::*;

pub fn spawn_map(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    let texture_handle = asset_server.load("mapa_dev.png");

    commands.spawn(SpriteBundle {
        texture: texture_handle,
        ..default()
    });
}
