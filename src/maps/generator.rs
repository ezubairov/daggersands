use crate::prelude::*;

use super::Map;

pub const MAP_WIDTH: usize = 80;
pub const MAP_HEIGHT: usize = 60;
pub const MAP_COUNT: usize = MAP_WIDTH * MAP_HEIGHT;

pub fn spawn_map(mut commands: Commands) {
    commands.init_resource::<Map>();
}
