use crate::prelude::*;
use std::collections::HashMap;

pub mod components;
mod systems;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentMap>()
            .add_systems(PostStartup, systems::spawn_map);
    }
}

#[derive(Default, Resource)]
pub struct CurrentMap {
    pub tiles: HashMap<IVec2, Entity>,
}

