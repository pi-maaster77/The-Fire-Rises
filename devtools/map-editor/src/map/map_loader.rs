use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MapExport {
    pub version: String,
    pub map_params: MapParams,
    pub seed_points: Vec<[f32; 2]>,
    pub regions: Vec<RegionData>,
    pub states: Vec<StateData>,
    pub provinces: Vec<ProvinceData>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct MapParams {
    pub width: u32,
    pub height: u32,
    pub voronoi_points: u32,
    pub lloyd_iterations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct RegionData {
    pub id: String,
    pub name: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct StateData {
    pub id: String,
    pub name: String,
    pub region_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)]
pub struct ProvinceData {
    pub id: String,
    pub seed_index: Option<usize>,
    pub state_id: String,
    pub region_id: Option<String>,
    pub center: [f32; 2],
}

/// Carga el mapa desde JSON
#[allow(dead_code)]
pub fn load_map_from_json(json_str: &str) -> Result<MapExport, Box<dyn std::error::Error>> {
    let map_export: MapExport = serde_json::from_str(json_str)?;
    Ok(map_export)
}

/// Exporta mapa a JSON compacto
#[allow(dead_code)]
pub fn export_to_compact_json(map: &MapExport) -> Result<String, Box<dyn std::error::Error>> {
    let json = serde_json::to_string(map)?;
    Ok(json)
}

