use crate::prelude::*;
use actions::get_action_at;
use bevy::ecs::system::SystemId;
use std::collections::VecDeque;

mod actions;
mod animation;
mod gamelog;
mod npcs;
mod player;

#[derive(Resource)]
struct QueueSystems {
    collect_actor_queue: SystemId,
    handle_actor_queue: SystemId,
}
impl FromWorld for QueueSystems {
    fn from_world(world: &mut World) -> Self {
        Self {
            collect_actor_queue: world.register_system(collect_actor_queue),
            handle_actor_queue: world.register_system(handle_actor_queue),
        }
    }
}

#[derive(Default, Resource)]
struct ActionQueue(VecDeque<Box<dyn actions::Action>>);

#[derive(Default, Resource)]
struct ActorQueue(VecDeque<Entity>);

#[allow(clippy::type_complexity)]
fn collect_actor_queue(
    query: Query<Entity, (With<Npc>, Without<Player>, Without<Dead>)>,
    player_query: Query<Entity, With<Player>>,
    mut queue: ResMut<ActorQueue>,
) {
    queue.0 = query.iter().collect();
    if let Ok(player) = player_query.single() {
        queue.0.push_front(player);
    }
}

fn handle_actor_queue(world: &mut World) {
    // if the queue is empty, start a new turn by collecting actors again
    let Some(&entity) = world.resource::<ActorQueue>().0.front() else {
        let _ = world.run_system(world.resource::<QueueSystems>().collect_actor_queue);
        return;
    };

    // check if player has an intent buffered
    if let Some(mut player) = world.get_mut::<Player>(entity) {
        if let Some(target) = player.0.take() {
            if let Some(action) = get_action_at(entity, target, world) {
                world.resource_mut::<ActionQueue>().0.push_back(action);
                world.resource_mut::<ActorQueue>().0.pop_front();
            }
        }
        return;
    }

    // otherwise handle npc actor
    world.resource_mut::<ActorQueue>().0.pop_front();

    if let Some(action) = npcs::get_npc_action(entity, world) {
        world.resource_mut::<ActionQueue>().0.push_back(action);
    }
}

fn handle_action_queue(world: &mut World) {
    if let Some(action) = world.resource_mut::<ActionQueue>().0.pop_front() {
        if action.is_valid(world) {
            let result = action.execute(world);
            if let Some(result) = result {
                world.resource_mut::<ActionQueue>().0.push_back(result);
            }
        }
    } else {
        // no more actions to execute - progress the actor queue
        let _ = world.run_system(world.resource::<QueueSystems>().handle_actor_queue);
    }
}

fn handle_input_events(
    mut events: EventReader<InputEvent>,
    mut query: Query<(&mut Player, &Position)>,
) {
    for event in events.read() {
        if let Ok((mut player, position)) = query.single_mut() {
            player.0 = Some(position.0 + event.0);
        }
    }
}

pub struct GamePlugin;
impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ActionQueue>()
            .init_resource::<ActorQueue>()
            .init_resource::<QueueSystems>()
            .add_systems(
                Update,
                (
                    handle_input_events,
                    animation::handle_game_events,
                    animation::handle_animations,
                    gamelog::handle_game_events,
                ),
            )
            .add_systems(Update, handle_action_queue.run_if(on_event::<GameTick>));
    }
}
