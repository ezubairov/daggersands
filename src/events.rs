use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct InputEvent(pub IVec2);

#[derive(Event)]
pub struct GameTick;

#[derive(Event, Debug)]
pub enum GameEvent {
    // Entity to move, (origin, destination)
    Move(Entity, (IVec2, IVec2)),
    Attack(Entity, (Entity, IVec2)),
    Damage(Entity, u32),
    // Killed entity, position of death
    Kill(Entity, IVec2),
}

#[derive(Event, Debug)]
pub struct GamelogEvent(pub String);
