use bevy::app::{App, Plugin};
use gamelog_ui::GamelogUIPlugin;
mod gamelog_ui;
use character_status_ui::CharacterStatusUIPlugin;
mod character_status_ui;

pub struct UIPlugin;
impl Plugin for UIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((GamelogUIPlugin, CharacterStatusUIPlugin));
    }
}
