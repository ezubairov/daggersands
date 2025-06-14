use crate::{game::ActorQueue, prelude::*};

pub fn get_action_at(entity: Entity, target: IVec2, world: &mut World) -> Option<Box<dyn Action>> {
    let actions: Vec<Box<dyn Action>> = vec![
        Box::new(MoveAction { entity, target }),
        Box::new(MeleeAction { entity, target }),
    ];
    actions.into_iter().find(|action| action.is_valid(world))
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>>;
    fn is_valid(&self, _world: &mut World) -> bool {
        true
    }
}

pub struct MoveAction {
    pub entity: Entity,
    pub target: IVec2,
}
impl Action for MoveAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        let mut original_position = world.get_mut::<Position>(self.entity)?;
        let origin = original_position.0;
        original_position.0 = self.target;

        // This triggers observers *right now*
        // We use this to reindex blocked tiles on a map
        world.trigger(GameEvent::Move(self.entity, (origin, self.target)));
        // This sends listeners and they might process event some time later in some order
        world.send_event::<GameEvent>(GameEvent::Move(self.entity, (origin, self.target)));

        None
    }
    fn is_valid(&self, world: &mut World) -> bool {
        let Some(map) = world.get_resource::<Map>() else {
            return false;
        };

        // Can't move if you are dead!
        if world.entity(self.entity).get::<Dead>().is_some() {
            return false
        }


        // Check if in bounds of a map
        if self.target.x < 1
            || self.target.x > map.width - 1
            || self.target.y < 1
            || self.target.y > map.height - 1
        {
            return false;
        }

        if map.blocked[map.xy_idx(self.target.x, self.target.y)] {
            return false;
        }

        true
    }
}

pub struct MeleeAction {
    pub entity: Entity,
    pub target: IVec2,
}
impl Action for MeleeAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        let &target = get_entities_with::<Health>(self.target, world).first()?;
        let damage = world.get::<Melee>(self.entity)?.damage;
        world.send_event::<GameEvent>(GameEvent::Attack(self.entity, (target, self.target)));
        Some(Box::new(DamageAction {
            entity: target,
            value: damage,
        }))
    }
    fn is_valid(&self, world: &mut World) -> bool {
        let Some(map) = world.get_resource::<Map>() else {
            return false;
        };

        // Can't attack if you are dead!
        if world.entity(self.entity).get::<Dead>().is_some() {
            return false
        }


        // Check if in bounds of a map
        if self.target.x < 1
            || self.target.x > map.width - 1
            || self.target.y < 1
            || self.target.y > map.height - 1
        {
            return false;
        }
        let mut targets = get_entities_with::<Health>(self.target, world);
        if targets.is_empty() {
            return false;
        }

        // disallow friendly fire
        targets.push(self.entity);
        let npcs = targets
            .iter()
            .filter(|&&e| world.get::<Npc>(e).is_some())
            .count();
        npcs == 1
    }
}

pub struct DamageAction {
    pub entity: Entity,
    pub value: u32,
}
impl Action for DamageAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        let mut health = world.get_mut::<Health>(self.entity)?;
        let hp_before_damage = health.hp;
        health.hp = health.hp.saturating_sub(self.value);
        let damage_taken = hp_before_damage - health.hp;
        if health.hp == 0 {
            return Some(Box::new(KillAction {
                entity: self.entity,
            }));
        }

        world.send_event::<GameEvent>(GameEvent::Damage(self.entity, damage_taken));
        None
    }
}

pub struct KillAction {
    pub entity: Entity,
}
impl Action for KillAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        world.send_event::<GameEvent>(GameEvent::Kill(self.entity));
        world
            .entity_mut(self.entity)
            .remove::<Health>()
            .insert(Dead);
        None
    }
}
