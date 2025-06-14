use std::collections::VecDeque;

use bevy::prelude::*;

pub enum AnimationKind {
    Translate(VecDeque<Vec3>),
}

#[derive(Component)]
pub struct Animation(pub AnimationKind);

#[derive(Component)]
pub struct Position(pub IVec2);

#[derive(Component)]
pub struct Health {
    pub hp: u32,
}

#[derive(Component)]
pub struct Dead;

#[derive(Component)]
pub struct Melee {
    pub damage: u32,
}

#[derive(Component)]
pub struct Npc;

#[derive(Component)]
pub struct BlocksTile;

#[derive(Component, Default)]
pub struct Player(pub Option<IVec2>);

#[derive(Component)]
pub struct Piece {
    pub kind: String,
    pub name: String,
}

#[derive(Component)]
pub struct Move;
