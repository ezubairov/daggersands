use crate::prelude::*;

#[derive(Bundle)]
pub struct PlayerBundle {
    piece: PieceBundle,
    combat_stats: CombatStatsBundle,
    marker: Player,
}

impl Default for PlayerBundle {
    fn default() -> PlayerBundle {
        PlayerBundle {
            marker: Player::default(),
            piece: PieceBundle {
                name: Name("Hero".into()),
                position: Position(IVec2::new(0, 0)),
                sprite_id: SpriteId(4),
            },
            combat_stats: CombatStatsBundle {
                health: Health { hp: 100 },
                melee: Melee { damage: 10 },
            },
        }
    }
}
