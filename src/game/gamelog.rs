use crate::prelude::*;

pub fn handle_game_events(
    query: Query<(Entity, &Piece)>,
    mut events: EventReader<GameEvent>,
    mut gamelog_event_writer: EventWriter<GamelogEvent>,
) {
    for event in events.read() {
        match event {
            GameEvent::Attack(actor, (target, _target_position)) => {
                let actor_name = &query.get(*actor).unwrap().1.name;
                let target_name = &query.get(*target).unwrap().1.name;
                let message = format!("{actor_name} attacks {target_name}!").to_string();
                gamelog_event_writer.write(GamelogEvent(message));
            }
            GameEvent::Damage(target, value) => {
                let target_name = &query.get(*target).unwrap().1.name;
                let message = format!("{target_name} takes {value} damage!").to_string();
                gamelog_event_writer.write(GamelogEvent(message));
            }
            GameEvent::Kill(entity, _) => {
                let entity_name = &query.get(*entity).unwrap().1.name;
                let message = format!("{entity_name} dies!").to_string();
                gamelog_event_writer.write(GamelogEvent(message));
            }
            _ => (),
        }
    }
}
