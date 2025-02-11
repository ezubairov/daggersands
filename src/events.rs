use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct InputEvent(pub IVec2);

#[derive(Event)]
pub struct GameTick;

#[derive(Event, Debug)]
pub enum GameEvent {
    Move(Entity, IVec2),
    Attack(Entity, IVec2),
}
