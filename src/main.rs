mod components;
mod events;
mod game;
mod globals;
mod graphics;
mod map;
mod utils;
mod input;

mod prelude {
    pub use crate::utils::*;
    pub use crate::components::*;
    pub use crate::events::*;
    pub use crate::globals::*;
    pub use bevy::prelude::*;
}

use graphics::tiles::TILE_SIZE;
use prelude::*;

fn setup(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Transform::from_translation(Vec3::new(10. * TILE_SIZE, 6. * TILE_SIZE, 0.)),
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
        .add_plugins(map::MapPlugin)
        .add_plugins(graphics::GraphicsPlugin)
        .add_event::<events::InputEvent>()
        .add_event::<events::GameEvent>()
        .add_event::<events::GameTick>()
        .add_systems(Update, input::handle_game_keyboard)
        .add_plugins(game::GamePlugin)
        .run();
}
