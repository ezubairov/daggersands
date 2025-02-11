use crate::prelude::*;

pub fn spawn_player(mut commands: Commands) {
    commands.spawn((
        Player::default(),
        Piece {
            kind: "Player".to_string(),
        },
        Position(IVec2::ZERO),
    ));
}
