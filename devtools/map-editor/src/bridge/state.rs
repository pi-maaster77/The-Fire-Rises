// devtools/map-editor/src/bridge/state.rs

use std::sync::Mutex;

use once_cell::sync::Lazy;
use lazy_static::lazy_static; // Asegurate de tener esta crate o usá Mutex::new(None) directamente

pub static MAP_IMAGE_DATA: Mutex<Option<(Vec<u8>, u32, u32)>> = Mutex::new(None);
lazy_static! {
    pub static ref EXTERNAL_SELECTION: Mutex<Option<String>> = Mutex::new(None);
}
pub static STATE_ASSIGNMENTS: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| Mutex::new(Vec::new()));
