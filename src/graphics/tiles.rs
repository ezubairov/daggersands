use super::{assets::SpriteTextures, tile_to_world};
use crate::{prelude::*, Map};

pub fn render_tiles(mut commands: Commands, map: Res<Map>, textures: Res<SpriteTextures>) {
    let Some(atlas) = textures.0.get("TileMap") else {
        return;
    };
    for x in 0..map.width {
        for y in 0..map.height {
            let position = IVec2::new(x, y);
            commands.spawn((
                Sprite::from_atlas_image(
                    atlas.texture.clone(),
                    TextureAtlas {
                        layout: atlas.layout.clone(),
                        index: 68,
                    },
                ),
                Transform::from_translation(tile_to_world(position, Some(0.)))
                    .with_scale(Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.)),
                Position(position),
            ));
        }
    }
}
