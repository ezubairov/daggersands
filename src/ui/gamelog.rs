use bevy::{color::palettes::css::BLUE, render::view::RenderLayers};

use crate::{camera::GamelogCamera, prelude::*};

#[derive(Component)]
pub struct GamelogRoot;

#[derive(Default, Resource)]
pub struct Gamelog {
    entries: Vec<String>,
}

fn setup_gamelog(camera: Query<Entity, With<GamelogCamera>>, mut commands: Commands) {
    commands.spawn((
        GamelogRoot,
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            border: UiRect::axes(Val::Vw(5.), Val::Vh(5.)),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BorderColor(BLUE.into()),
        RenderLayers::layer(28),
        UiTargetCamera(camera.single().unwrap()),
    ));
}

fn render_gamelog(
    mut commands: Commands,
    gamelog_root: Query<Entity, With<GamelogRoot>>,
    gamelog: Res<Gamelog>,
) {
    commands
        .entity(gamelog_root.single().unwrap())
        .with_children(|parent| {
            for i in gamelog.entries.iter() {
                parent.spawn((Text::new(format!("Foobar {i}")), Label));
            }
        });
}

pub struct GamelogPlugin;
impl Plugin for GamelogPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gamelog>()
            // PostStartup as we need cameras to be already set up, but that's a hack
            // TODO: look for a way of defining dependency/order on a different plugin
            .add_systems(PostStartup, setup_gamelog)
            .add_systems(Update, render_gamelog);
    }
}
