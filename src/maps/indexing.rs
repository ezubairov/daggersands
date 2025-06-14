use crate::GameEvent;
use bevy::prelude::*;

use super::Map;

pub fn index_map(trigger: Trigger<GameEvent>, mut map: ResMut<Map>) {
    let event = trigger.event();
    println!("{:#?}", event);
    match event {
        GameEvent::Move(_, (origin, destination)) => {
            let origin_idx = map.ivec2_to_idx(*origin);
            map.blocked[origin_idx] = false;
            let destination_idx = map.ivec2_to_idx(*destination);
            map.blocked[destination_idx] = true
        }
        GameEvent::Kill(_, position) => {
            let death_position = map.ivec2_to_idx(*position);
            println!("{}", map.blocked[death_position]);
            map.blocked[death_position] = false
        }
        _ => (),
    }
}
