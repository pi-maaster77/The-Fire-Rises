// devtools/map-editor/src/map/systems/events.rs
#[derive(Event)]
pub struct AssignStateToRegionEvent {
    pub state_id: String,
    pub region_id: String,
}