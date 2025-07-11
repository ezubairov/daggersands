pub fn spawn_npcs(mut commands: Commands) {
    commands.spawn((
        Npc,
        Piece,
        Position(IVec2::new(3, 5)),
        Move,
        BlocksTile,
        Health { hp: 20 },
        Melee { damage: 2 },
    ));
    commands.spawn((
        Npc,
        Piece,
        Position(IVec2::new(5, 5)),
        Move,
        BlocksTile,
        Health { hp: 20 },
        Melee { damage: 2 },
    ));
}

use crate::{pieces::monsters::NpcCombatantBundle, prelude::*};

#[derive(Bundle)]
pub struct OrcBundle(NpcCombatantBundle);
impl core::ops::Deref for OrcBundle {
    type Target = NpcCombatantBundle;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl OrcBundle {
    fn new(&self) -> OrcBundle {

        self.0.new()
    }
}

impl Default for OrcBundle {
    fn default() -> OrcBundle {
        OrcBundle {}
    }
}

