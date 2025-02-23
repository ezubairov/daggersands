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

use bevy::window::WindowResolution;
use prelude::*;

fn main() {
    #[cfg(target_arch = "wasm32")]
    console_error_panic_hook::set_once();

    App::new()
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                        ..Default::default()
                    }),
                    ..Default::default()
                })
                .set(ImagePlugin::default_nearest()),
        )
        .add_plugins((
            maps::MapPlugin,
            camera::CameraPlugin,
            graphics::GraphicsPlugin,
            game::GamePlugin,
            input::InputPlugin,
        ))
        .add_event::<events::InputEvent>()
        .add_event::<events::GameEvent>()
        .add_event::<events::GameTick>()
        .run();
}
