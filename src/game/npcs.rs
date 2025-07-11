use crate::prelude::*;

use super::actions::{get_action_at, Action};

pub fn get_npc_action(entity: Entity, world: &mut World) -> Option<Box<dyn Action>> {
    let mut npc_position = world.get_mut::<Position>(entity)?.0;
    let player_position = world
        .query_filtered::<&Position, With<Player>>()
        .single(world)
        .unwrap()
        .0;
    let map = world.get_resource::<Map>()?;
    let path = bracket_pathfinding::prelude::a_star_search(
        map.ivec2_to_idx(npc_position),
        map.ivec2_to_idx(player_position),
        map,
    );

    if path.success && path.steps.len() > 1 {
        npc_position.x = path.steps[1] as i32 % map.width;
        npc_position.y = path.steps[1] as i32 / map.width;
        if let Some(action) = get_action_at(entity, npc_position, world) {
            return Some(action);
        }
    }

    None
}
