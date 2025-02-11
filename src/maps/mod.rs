use crate::prelude::*;

mod map;
pub use map::Map;
mod generator;
mod indexing;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generator::spawn_map)
            .add_observer(indexing::index_map);
    }
}
