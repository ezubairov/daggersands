use crate::prelude::*;
mod player;
use bevy::platform::collections::HashMap;
use leafwing_manifest::{
    asset_state::SimpleAssetState,
    identifier::Id,
    manifest::{Manifest, ManifestFormat},
    plugin::{ManifestPlugin, RegisterManifest},
};
pub use player::PlayerBundle;
use serde::{Deserialize, Serialize};
mod monsters;

#[derive(Bundle)]
pub struct PieceBundle {
    position: Position,
    // Rust doesn't like Name, so asks for explicit import
    name: crate::components::Name,
    sprite_id: SpriteId,
}

#[derive(Bundle)]
pub struct CombatStatsBundle {
    health: Health,
    melee: Melee,
}

#[derive(Asset, Serialize, Deserialize, TypePath, Debug, PartialEq)]
pub struct RawMonsterManifest {}

pub struct Monster {}
pub struct RawMonster {}

#[derive(Resource)]
pub struct MonsterManifest {
    monsters: HashMap<Id<Monster>, Monster>,
}

impl Manifest for MonsterManifest {
    type RawManifest = RawMonsterManifest;

    type RawItem = RawMonster;

    type Item = Monster;

    type ConversionError = std::convert::Infallible;

    const FORMAT: ManifestFormat = ManifestFormat::Ron;

    fn from_raw_manifest(
        raw_manifest: Self::RawManifest,
        world: &mut World,
    ) -> std::result::Result<Self, Self::ConversionError> {
        todo!()
    }

    fn get(&self, id: Id<Self::Item>) -> Option<&Self::Item> {
        self.monsters.get(&id)
    }
}

pub struct PiecesPlugin;
impl Plugin for PiecesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SimpleAssetState>()
            .add_plugins(ManifestPlugin::<SimpleAssetState>::default())
            .register_manifest::<MonsterManifest>("manifests/monsters.ron");
    }
}
