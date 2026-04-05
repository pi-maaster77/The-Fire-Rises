// devtools/map-editor/src/map/systems/camera.rs

use bevy::prelude::*;
use bevy::input::mouse::{MouseWheel, MouseMotion};
use crate::map::components::CameraConfig;

pub fn camera_control_system(
    time: Res<Time>,
    config: Res<CameraConfig>,
    mouse_button: Res<ButtonInput<MouseButton>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut mouse_wheel: EventReader<MouseWheel>,
    mut mouse_motion: EventReader<MouseMotion>,
    // Necesitamos la ventana para la posición del mouse
    q_window: Query<&Window>,
    mut query: Query<(&mut Transform, &mut OrthographicProjection), With<Camera2d>>,
) {
    let (mut transform, mut projection) = query.single_mut();
    let window = q_window.single();

    // --- 1. ZOOM AL PUNTERO ---
    for event in mouse_wheel.read() {
        let zoom_factor = if event.y > 0.0 {
            1.0 - config.zoom_speed
        } else {
            1.0 + config.zoom_speed
        };

        // Si el mouse está en la ventana, hacemos zoom hacia él
        if let Some(cursor_pos) = window.cursor_position() {
            // Convertimos posición del cursor (pantalla) a coordenadas relativas al centro
            let window_size = Vec2::new(window.width(), window.height());
            let mut mouse_rel = cursor_pos - window_size / 2.0;
						mouse_rel.y = -mouse_rel.y;
            
            // Punto en el mundo antes del zoom
            let mouse_world_before = transform.translation.truncate() + mouse_rel * projection.scale;

            // Aplicamos el nuevo zoom
            projection.scale = (projection.scale * zoom_factor).clamp(0.05, 10.0);

            // Ajustamos la posición para que el mouse siga apuntando al mismo lugar del mapa
            let mouse_world_after = transform.translation.truncate() + mouse_rel * projection.scale;
            let diff = mouse_world_before - mouse_world_after;
            
            transform.translation += diff.extend(0.0);
        } else {
            // Si el mouse no está (raro), zoom al centro normal
            projection.scale = (projection.scale * zoom_factor).clamp(0.05, 10.0);
        }
    }

    // --- 2. MOVIMIENTO WASD ---
    let mut velocity = Vec3::ZERO;
    if keyboard.pressed(KeyCode::KeyW) { velocity.y += 1.0; }
    if keyboard.pressed(KeyCode::KeyS) { velocity.y -= 1.0; }
    if keyboard.pressed(KeyCode::KeyA) { velocity.x -= 1.0; }
    if keyboard.pressed(KeyCode::KeyD) { velocity.x += 1.0; }

    if velocity != Vec3::ZERO {
        transform.translation += velocity.normalize() 
            * config.move_speed 
            * projection.scale 
            * time.delta_seconds();
    }

    // --- 3. PANEO CON CLICK DERECHO ---
    if mouse_button.pressed(config.pan_button) {
        for event in mouse_motion.read() {
            transform.translation.x -= event.delta.x * projection.scale;
            transform.translation.y += event.delta.y * projection.scale;
        }
    }
}