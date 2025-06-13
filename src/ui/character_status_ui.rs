use bevy::{color::palettes::css::WHITE, render::view::RenderLayers};

use crate::{camera::CharacterStatusCamera, prelude::*};

#[derive(Component)]
pub struct CharacterStatusUIRoot;

fn setup_character_status_ui(
    camera: Query<Entity, With<CharacterStatusCamera>>,
    mut commands: Commands,
) {
    commands
        .spawn((
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                padding: UiRect::all(Val::Px(3.)),
                flex_direction: FlexDirection::Row,
                ..default()
            },
            BorderColor(WHITE.into()),
            RenderLayers::layer(28),
            UiTargetCamera(camera.single().unwrap()),
        ))
        .with_child((
            CharacterStatusUIRoot,
            Node {
                flex_direction: FlexDirection::Row,
                align_self: AlignSelf::Stretch,
                overflow: Overflow::scroll_y(),
                border: UiRect::all(Val::Px(3.)),
                flex_grow: 1.,
                ..default()
            },
            BorderColor(WHITE.into()),
        ));
}

pub struct CharacterStatusUIPlugin;
impl Plugin for CharacterStatusUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, setup_character_status_ui);
    }
}
