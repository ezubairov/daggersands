use crate::{map::components::Tile, prelude::*};
use super::{assets::SpriteTextures, tile_to_world};

pub const TILE_SIZE: f32 = 32.;

pub fn render_tiles(
    mut commands: Commands,
    query: Query<(Entity, &Position), Added<Tile>>,
    textures: Res<SpriteTextures>,
) {
    let Some(atlas) = textures.0.get("TileMap") else {
        return;
    };
    for (entity, position) in query.iter() {
        commands.entity(entity)
            .insert((
                Sprite::from_atlas_image(
                    atlas.texture.clone(),
                    TextureAtlas {
                        layout: atlas.layout.clone(),
                        index: 68,
                    },
                ),
                Transform::from_translation(tile_to_world(position.0, Some(0.)))
                    .with_scale(Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.)),
                Position(position.0),
            ));
            }
}
