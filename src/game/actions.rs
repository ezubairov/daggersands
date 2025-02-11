use crate::{map::CurrentMap, prelude::*};

pub fn get_action_at(entity: Entity, target: IVec2, world: &mut World) -> Option<Box<dyn Action>> {
    let actions: Vec<Box<dyn Action>> = vec![
        Box::new(MoveAction { entity, target }),
        // Box::new(AttackAction { entity, target }),
    ];
    actions.into_iter().find(|action| action.is_valid(world))
}

pub trait Action: Send + Sync {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>>;
    fn is_valid(&self, world: &mut World) -> bool {
        true
    }
}

pub struct MoveAction {
    pub entity: Entity,
    pub target: IVec2,
}
impl Action for MoveAction {
    fn execute(&self, world: &mut World) -> Option<Box<dyn Action>> {
        world.get_mut::<Position>(self.entity)?.0 = self.target;
        world.send_event::<GameEvent>(GameEvent::Move(self.entity, self.target));
        None
    }
    fn is_valid(&self, world: &mut World) -> bool {
        let Some(board) = world.get_resource::<CurrentMap>() else {
            return false;
        };
        if !board.tiles.contains_key(&self.target) {
            return false;
        }
        // if !get_entities_with::<Obstacle>(self.target, world).is_empty() {
        //     return false;
        // }
        true
    }
}
