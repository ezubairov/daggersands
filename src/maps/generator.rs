use crate::prelude::*;

use super::{map::TileType, Map};

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 60;
const MAP_COUNT: usize = MAP_WIDTH * MAP_HEIGHT;

pub fn spawn_map(mut commands: Commands) {
    commands.insert_resource(Map {
        width: MAP_WIDTH as i32,
        height: MAP_HEIGHT as i32,
        tiles: vec![TileType::Floor; MAP_COUNT],
        blocked: vec![false; MAP_COUNT],
    })
}
