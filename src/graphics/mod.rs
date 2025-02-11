use crate::prelude::*;

mod assets;
mod pieces;
pub mod tiles;

pub struct GraphicsPlugin;

impl Plugin for GraphicsPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(assets::SpriteTextures::default())
            .add_systems(Startup, assets::load_assets)
            .add_systems(
                Update,
                (
                    tiles::render_tiles,
                    pieces::render_pieces,
                    pieces::update_piece_position,
                ),
            );
    }
}
