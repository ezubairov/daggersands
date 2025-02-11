use crate::GameEvent;
use bevy::prelude::*;

use super::Map;

pub fn index_map(trigger: Trigger<GameEvent>, mut map: ResMut<Map>) {
    let event = trigger.event();
    if let GameEvent::Move(_, _) = event {
        map.populate_blocked()
    }
}
