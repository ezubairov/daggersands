use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, map: Res<Map>) {
    commands.spawn((
        Player::default(),
        Piece {
            kind: "Player".to_string(),
            name: "Player".to_string(),
        },
        Position(IVec2::new(1, 1)),
        Health { hp: 100 },
        Melee { damage: 10 },
    ));
}
