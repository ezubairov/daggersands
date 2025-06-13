use bevy::app::{App, Plugin};
use gamelog::GamelogPlugin;
mod gamelog;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GamelogPlugin);
    }
}
