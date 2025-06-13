use bevy::{
    color::palettes::css::{BLACK, RED},
    render::{
        camera::{ScalingMode, Viewport},
        view::RenderLayers,
    },
};

use crate::prelude::*;

// Setup cameras in a following setup:
// ┌───────┬───┐
// │ Main  │   │
// ├───────┤ UI│
// │Gamelog│   │
// └───────┴───┘

#[derive(Component)]
#[require(Camera2d)]
pub struct MainCamera;

#[derive(Component)]
#[require(Camera2d)]
pub struct UICamera;

#[derive(Component)]
#[require(Camera2d)]
pub struct GamelogCamera;

fn setup_main_camera(mut commands: Commands) {
    commands.spawn((
        MainCamera,
        RenderLayers::from_layers(&[0]),
        Camera {
            // Lower than everything else
            order: 0,
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),

                // No idea why we actually have to multiply here, probably something with scaling
                physical_size: UVec2::new(
                    2 * GAME_VIEWPORT_WIDTH as u32,
                    2 * GAME_VIEWPORT_HEIGHT as u32,
                ),
                ..Default::default()
            }),
            clear_color: ClearColorConfig::Custom(BLACK.into()),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(
            0.5 * GAME_VIEWPORT_WIDTH,
            0.5 * GAME_VIEWPORT_HEIGHT,
            0.,
        )),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 2.,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn setup_ui_camera(mut commands: Commands) {
    let camera = commands
        .spawn((
            UICamera,
            RenderLayers::from_layers(&[30]),
            Camera {
                // Higher than everything else
                order: 2,
                viewport: Some(Viewport {
                    physical_position: UVec2::new(
                        2 * (WINDOW_WIDTH - UI_VIEWPORT_WIDTH) as u32,
                        (WINDOW_HEIGHT - UI_VIEWPORT_HEIGHT) as u32,
                    ),

                    // No idea why we actually have to multiply here, probably something with scaling
                    physical_size: UVec2::new(
                        2 * UI_VIEWPORT_WIDTH as u32,
                        2 * UI_VIEWPORT_HEIGHT as u32,
                    ),
                    ..Default::default()
                }),
                clear_color: ClearColorConfig::None,
                ..Default::default()
            },
        ))
        .id();

    commands.spawn((
        Node {
            width: Val::Vw(100.),
            height: Val::Vh(100.),
            border: UiRect::axes(Val::Vw(5.), Val::Vh(5.)),
            flex_wrap: FlexWrap::Wrap,
            ..default()
        },
        BorderColor(RED.into()),
        RenderLayers::layer(30),
        TargetCamera(camera),
    ));
}

fn setup_gamelog_camera(mut commands: Commands) {
    commands.spawn((
        GamelogCamera,
        RenderLayers::from_layers(&[28]),
        Camera {
            // Higher than main, lower than UI
            order: 1,
            viewport: Some(Viewport {
                physical_position: UVec2::new(
                    0,
                    2 * (WINDOW_HEIGHT - GAMELOG_VIEWPORT_HEIGHT) as u32,
                ),

                // No idea why we actually have to multiply here, probably something with scaling
                physical_size: UVec2::new(
                    2 * GAMELOG_VIEWPORT_WIDTH as u32,
                    2 * GAMELOG_VIEWPORT_HEIGHT as u32,
                ),
                ..Default::default()
            }),
            clear_color: ClearColorConfig::None,
            ..Default::default()
        },
    ));
}

fn track_player(
    player_query: Query<&Position, (Changed<Position>, With<Player>)>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) {
    for player_position in player_query.iter() {
        let mut camera_transform = camera_query.single_mut();
        camera_transform.translation = tile_to_world(player_position.0, None);
    }
}

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (setup_main_camera, setup_gamelog_camera, setup_ui_camera),
        )
        .add_systems(Update, track_player.run_if(on_event::<GameTick>));
    }
}
