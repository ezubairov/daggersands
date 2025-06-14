use crate::GameEvent;
use bevy::prelude::*;

use super::Map;

pub fn index_map(trigger: Trigger<GameEvent>, mut map: ResMut<Map>) {
    let event = trigger.event();
    if let GameEvent::Move(_, (origin, destination)) = event {
        let origin_idx = map.ivec2_to_idx(*origin);
        map.blocked[origin_idx] = false;
        let destination_idx = map.ivec2_to_idx(*destination);
        map.blocked[destination_idx] = true
    }
}
