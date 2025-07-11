use crate::prelude::*;

use super::assets::SpriteTextures;

pub const PIECE_SPEED: f32 = 10.;
pub const POSITION_TOLERANCE: f32 = 0.1;
pub fn render_pieces(
    mut commands: Commands,
    query: Query<(Entity, &Position, &SpriteId), Added<Piece>>,
    textures: Res<SpriteTextures>,
) {
    let Some(atlas) = textures.0.get("TileMap") else {
        return;
    };
    for (entity, position, sprite_idx) in query.iter() {
        commands.entity(entity).insert((
            Sprite::from_atlas_image(
                atlas.texture.clone(),
                TextureAtlas {
                    layout: atlas.layout.clone(),
                    index: sprite_idx.0,
                },
            ),
            Transform::from_translation(super::tile_to_world(position.0, Some(0.5)))
                .with_scale(Vec3::new(SPRITE_SCALE, SPRITE_SCALE, 1.)),
        ));
    }
}

pub fn update_piece_position(
    mut query: Query<(&Position, &mut Transform), With<Piece>>,
    time: Res<Time>,
) {
    for (position, mut transform) in query.iter_mut() {
        let target = super::tile_to_world(position.0, Some(0.5));
        let d = (target - transform.translation).length();
        if d > POSITION_TOLERANCE {
            transform.translation = transform
                .translation
                .lerp(target, PIECE_SPEED * time.delta_secs());
        } else {
            transform.translation = target;
        }
    }
}

pub fn hide_dead_pieces(mut commands: Commands, query: Query<Entity, Added<Dead>>) {
    for entity in query.iter() {
        commands.entity(entity).remove::<Sprite>();
    }
}
