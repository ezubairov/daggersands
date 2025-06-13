use crate::prelude::*;

pub fn spawn_player(mut commands: Commands, map: Res<Map>) {
    commands.spawn((
        Player::default(),
        Piece {
            kind: "Player".to_string(),
        },
        Position(IVec2::new(map.width / 2, map.height / 2)),
    ));
}
