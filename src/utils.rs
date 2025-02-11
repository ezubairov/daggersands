use crate::prelude::*;

pub fn tile_to_world(v: IVec2, z: Option<f32>) -> Vec3 {
    SPRITE_SIZE * Vec3::new(v.x as f32, v.y as f32, z.unwrap_or_default())
}
