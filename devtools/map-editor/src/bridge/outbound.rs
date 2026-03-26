// devtools/map-editor/src/bridge/outbound.rs

pub fn send_to_vue(event_type: &str, payload: &serde_json::Value) {
    let payload_str = serde_json::to_string(payload).unwrap();
    let js_code = format!("if (window.__BEVY_BRIDGE_INBOUND__) {{ window.__BEVY_BRIDGE_INBOUND__({{type: '{}', payload: {}}}); }}", event_type, payload_str);
    js_sys::eval(&js_code).unwrap();
}