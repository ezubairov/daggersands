use bevy::ecs::resource::Resource;
use bracket_random::rand;

#[derive(Resource)]
pub struct Rng(pub bracket_random::prelude::RandomNumberGenerator);
impl Rng {
    pub fn roll_dice(&mut self, n: i32, die_type: i32) -> i32 {
        self.0.roll_dice(n, die_type)
    }

    pub fn range<T>(&mut self, min: T, max: T) -> T
    where
        T: rand::distributions::uniform::SampleUniform + PartialOrd,
    {
        self.0.range(min, max)
    }
}
