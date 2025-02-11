use crate::prelude::*;

pub fn camera_move(
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
) {
    for player_position in player_query.iter() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation = tile_to_world(player_position.0, None);
    }
}
