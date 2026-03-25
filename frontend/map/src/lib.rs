// frontend/map/src/lib.rs

use bevy::prelude::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[derive(Resource)]
pub struct Counter(pub i32);

#[derive(Component)]
pub struct CounterText;

#[wasm_bindgen(start)]
pub fn run_app() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                // El ID debe coincidir con el <canvas id="canvas">
                canvas: Some("#canvas".into()),
                // Evita que el navegador use el scroll al tocar el canvas
                prevent_default_event_handling: true,
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ClearColor(Color::rgb(0.2, 0.2, 0.2))) // Fondo gris oscuro
        .insert_resource(Counter(0))
        .add_systems(Startup, setup)
        .add_systems(Update, (handle_input, handle_button_click, update_ui))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Texto del contador
    commands.spawn((
        TextBundle::from_section(
            "Contador: 0",
            TextStyle { font_size: 60.0, color: Color::WHITE, ..default() },
        ).with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
        CounterText,
    ));

    // Botón
    commands.spawn(ButtonBundle {
        style: Style {
            width: Val::Px(80.0),
            height: Val::Px(80.0),
            position_type: PositionType::Absolute,
            bottom: Val::Px(50.0),
            right: Val::Px(50.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        background_color: BackgroundColor(Color::rgb(0.2, 0.6, 0.2)),
        ..default()
    })
    .with_children(|parent| {
        parent.spawn(TextBundle::from_section(
            "+",
            TextStyle { font_size: 40.0, color: Color::WHITE, ..default() },
        ));
    });
}

// --- SISTEMAS DE LÓGICA ---

fn handle_input(keyboard: Res<ButtonInput<KeyCode>>, mut counter: ResMut<Counter>) {
    if keyboard.just_pressed(KeyCode::Space) { counter.0 += 1; }
}

fn update_ui(counter: Res<Counter>, mut query: Query<&mut Text, With<CounterText>>) {
    if counter.is_changed() {
        for mut text in &mut query {
            text.sections[0].value = format!("Contador: {}", counter.0);
        }
    }
}

fn handle_button_click(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<Button>)>,
    mut counter: ResMut<Counter>,
) {
    for (interaction, mut color) in &mut interaction_query {
        if *interaction == Interaction::Pressed {
            counter.0 += 1;
            *color = BackgroundColor(Color::rgb(0.4, 0.8, 0.4));
        } else {
            *color = BackgroundColor(Color::rgb(0.2, 0.6, 0.2));
        }
    }
}