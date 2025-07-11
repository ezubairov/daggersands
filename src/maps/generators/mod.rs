mod simple_map;
use crate::prelude::*;
use simple_map::SimpleMapGenerator;
mod utils;

use super::Map;

pub trait MapGenerator {
    fn build_map(&mut self, rng: ResMut<Rng>);
    fn spawn_entities(&mut self, commands: Commands);
    fn get_map(&self) -> Map;
}

pub fn spawn_map(mut commands: Commands, mut rng: ResMut<Rng>) {
    let build_roll = rng.roll_dice(1, 10);
    let mut builder = match build_roll {
        _ => SimpleMapGenerator::new(0),
    };

    builder.build_map(rng);
    commands.insert_resource(builder.get_map());

    builder.spawn_entities(commands);
}
