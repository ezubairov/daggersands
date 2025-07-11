use std::collections::VecDeque;

use bevy::prelude::*;

pub enum AnimationKind {
    Translate(VecDeque<Vec3>),
}

#[derive(Component)]
pub struct Animation(pub AnimationKind);

/// Stores positon of the entity on the map
#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Health {
    pub hp: u32,
}

/// Stores name of an entity
#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Melee {
    pub damage: u32,
}

#[derive(Component)]
pub struct Npc;

/// Marks that an entity is blocking movement through it's tile
/// Probably a good idea to add to an entity that also has Position component
#[derive(Component)]
pub struct BlocksTile;

/// Player marker
///
/// (Intent, for some reason)
#[derive(Component, Default)]
pub struct Player(pub Option<IVec2>);

/// Marks that an entity should have visual placement
#[derive(Component)]
pub struct Piece;

#[derive(Component)]
pub struct Move;

#[derive(Component)]
pub struct SpriteId(pub usize);
