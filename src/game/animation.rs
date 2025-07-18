use crate::prelude::*;

pub fn handle_game_events(
    mut commands: Commands,
    mut events: EventReader<GameEvent>,
    query: Query<&Transform>,
) {
    for event in events.read() {
        match event {
            GameEvent::Move(entity, (_origin, target)) => {
                commands
                    .entity(*entity)
                    .insert(Animation(AnimationKind::Translate(
                        vec![tile_to_world(*target, None)].into(),
                    )));
            }
            GameEvent::Attack(entity, (_target, target_position)) => {
                if let Ok(transform) = query.get(*entity) {
                    let current = transform.translation;
                    let target_v = tile_to_world(*target_position, None);
                    commands
                        .entity(*entity)
                        .insert(Animation(AnimationKind::Translate(
                            vec![0.5 * (current + target_v), current].into(),
                        )));
                }
            }
            _ => (),
        }
    }
}

pub fn handle_animations(
    mut commands: Commands,
    mut query: Query<(Entity, &mut Animation, &mut Transform)>,
    time: Res<Time>,
    mut events: EventWriter<GameTick>,
) {
    let mut is_animating = false;

    for (entity, mut animation, mut transform) in query.iter_mut() {
        match &mut animation.0 {
            AnimationKind::Translate(path) => {
                if path.is_empty() {
                    commands.entity(entity).remove::<Animation>();
                    continue;
                }

                is_animating = true;

                let origin = transform.translation;
                let target = Vec3::new(path[0].x, path[0].y, origin.z);
                let current = move_towards(origin, target, time.delta_secs() * MOVE_SPEED);

                if (path[0].with_z(0.) - current.with_z(0.)).length() < 0.1 {
                    transform.translation = path[0].with_z(origin.z);
                    path.pop_front();
                } else {
                    transform.translation = current;
                }
            }
        }
    }

    if !is_animating {
        events.write(GameTick);
    }
}

fn move_towards(origin: Vec3, target: Vec3, max_delta: f32) -> Vec3 {
    let a = target - origin;
    let l = a.length();
    if l <= max_delta || l == 0. {
        return target;
    }
    origin + a / l * max_delta
}
