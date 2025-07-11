use crate::prelude::*;

mod generators;
mod map;
pub use map::Map;
mod indexing;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, generators::spawn_map)
            .add_observer(indexing::index_map);
    }
}
