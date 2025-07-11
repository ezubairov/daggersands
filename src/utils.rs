use crate::prelude::*;

pub fn tile_to_world(v: IVec2, z: Option<f32>) -> Vec3 {
    SPRITE_SIZE * Vec3::new(v.x as f32, v.y as f32, z.unwrap_or_default())
}

pub fn get_entities_with<T: Component>(v: IVec2, world: &mut World) -> Vec<Entity> {
    world
        .query_filtered::<(Entity, &Position), With<T>>()
        .iter(world)
        .filter(|(_, p)| p.0 == v)
        .map(|(e, _)| e)
        .collect()
}

