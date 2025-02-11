mod camera;
mod components;
mod events;
mod game;
mod globals;
mod graphics;
mod input;
mod maps;
mod utils;

mod prelude {
    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::globals::*;
    pub use crate::maps::Map;
    pub use crate::utils::*;
    pub use bevy::prelude::*;
}

use bevy::render::camera::{ScalingMode, Viewport};
use camera::camera_move;
use maps::MapPlugin;
use prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(0, 0),
                // No idea why we actually have to multiply here, probably something with scaling
                // or something
                physical_size: UVec2::new(2 * GAME_VIEWPORT_WIDTH as u32, 2 * GAME_VIEWPORT_HEIGHT as u32),
                ..Default::default()
            }),
            ..Default::default()
        },
        Transform::from_translation(Vec3::new(0.5 * GAME_VIEWPORT_WIDTH, 0.5 * GAME_VIEWPORT_HEIGHT, 0.)),
        Projection::from(OrthographicProjection {
            scaling_mode: ScalingMode::WindowSize,
            scale: 2.,
            ..OrthographicProjection::default_2d()
        }),
    ));
}

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_systems(Startup, setup)
        .add_plugins(MapPlugin)
        .add_plugins(graphics::GraphicsPlugin)
        .add_event::<events::InputEvent>()
        .add_event::<events::GameEvent>()
        .add_event::<events::GameTick>()
        .add_systems(Update, input::handle_game_keyboard)
        .add_plugins(game::GamePlugin)
        .add_systems(Update, camera_move.run_if(on_event::<GameTick>))
        .run();
}
