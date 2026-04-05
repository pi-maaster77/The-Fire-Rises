// devtools/map-editor/src/bridge/state.rs

use std::sync::Mutex;

use once_cell::sync::Lazy;
use lazy_static::lazy_static; // Asegurate de tener esta crate o usá Mutex::new(None) directamente

pub static MAP_IMAGE_DATA: Mutex<Option<(Vec<u8>, u32, u32)>> = Mutex::new(None);
pub static STATE_ASSIGNMENTS: Lazy<Mutex<Vec<(String, String)>>> = Lazy::new(|| Mutex::new(Vec::new()));
pub static BRUSH_UPDATE: Lazy<Mutex<Option<(bool, Option<String>)>>> = Lazy::new(|| Mutex::new(None));

lazy_static! {
    pub static ref EXTERNAL_SELECTION: Mutex<Option<String>> = Mutex::new(None);
		pub static ref ACTIVE_REGION_UPDATE: Mutex<Option<Option<String>>> = Mutex::new(None);
		pub static ref REGION_CREATION: Mutex<Option<(String, String)>> = Mutex::new(None);
		pub static ref STATE_CREATION: Mutex<Option<(String, String, String)>> = Mutex::new(None);
		pub static ref EXPORT_TRIGGER: Mutex<bool> = Mutex::new(false);
}