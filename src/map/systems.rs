use crate::prelude::*;
use std::collections::HashMap;

use super::components::Tile;
use super::CurrentMap;

pub fn spawn_map(mut commands: Commands, mut current: ResMut<CurrentMap>) {
    current.tiles = HashMap::new();
    for x in 0..8 {
        for y in 0..8 {
            let v = IVec2::new(x, y);
            let tile = commands.spawn((Position(v), Tile)).id();
            current.tiles.insert(v, tile);
        }
    }
}
