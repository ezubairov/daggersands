use crate::prelude::*;
mod orc;

#[derive(Bundle)]
pub struct NpcCombatantBundle {
    piece: PieceBundle,
    combat_stats: CombatStatsBundle,
    marker: Npc,
}

