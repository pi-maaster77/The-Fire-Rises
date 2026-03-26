// devtools/map-editor/src/map/systems/setup.rs

use bevy::{

	ecs::system::Commands, 

};
use crate::map::components::{MapImage, ProvincePixelMap, SelectedProvinceId};

pub fn spawn_map(mut commands: Commands) {
    commands.insert_resource(MapImage {
        width: 0,
        height: 0,
        data: vec![],
    });
    commands.insert_resource(ProvincePixelMap {
        width: 0,
        height: 0,
        data: vec![],
    });
		commands.insert_resource(SelectedProvinceId(None));
}
