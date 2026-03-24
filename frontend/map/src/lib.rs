// frontend/map/src/lib.rs

use bevy::prelude::*;
use wasm_bindgen::prelude::*;
use js_sys::Reflect; // Necesario para manipular propiedades de JS

#[derive(Resource, Default)]
struct GlobalCounter(u32);

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__BEVY_BRIDGE__"])]
    fn on_bevy_counter_update(val: u32);
}

#[wasm_bindgen]
pub fn send_to_bevy(val: u32) {
    let window = web_sys::window().expect("no global `window` exists");
    // Correcto: usamos Reflect para escribir en window.next_counter_val
    let key = JsValue::from_str("next_counter_val");
    let value = JsValue::from_f64(val as f64);
    Reflect::set(&window, &key, &value).expect("failed to set window property");
}

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GlobalCounter>()
           .add_systems(Update, (read_js_updates, sync_to_vue));
    }
}

fn read_js_updates(mut counter: ResMut<GlobalCounter>) {
    let window = web_sys::window().unwrap();
    let key = JsValue::from_str("next_counter_val");
    
    // Reflect::get devuelve un Result<JsValue, JsValue>
    if let Ok(val) = Reflect::get(&window, &key) {
        if !val.is_null() && !val.is_undefined() {
            if let Some(num) = val.as_f64() {
                let new_val = num as u32;
                if counter.0 != new_val {
                    counter.0 = new_val;
                    // Limpiamos el buzón seteándolo a null
                    Reflect::set(&window, &key, &JsValue::NULL).unwrap();
                }
            }
        }
    }
}

fn sync_to_vue(counter: Res<GlobalCounter>) {
    if counter.is_changed() {
        on_bevy_counter_update(counter.0);
    }
}