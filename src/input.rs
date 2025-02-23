use crate::prelude::*;

pub fn handle_game_keyboard(
    input: Res<ButtonInput<KeyCode>>,
    mut input_events: EventWriter<InputEvent>,
) {
    if input.any_just_released([
        KeyCode::KeyW,
        KeyCode::ArrowUp,
        KeyCode::KeyK,
        KeyCode::Numpad8,
    ]) {
        input_events.send(InputEvent(IVec2::Y));
    }
    if input.any_just_released([
        KeyCode::KeyS,
        KeyCode::ArrowDown,
        KeyCode::KeyJ,
        KeyCode::Numpad2,
    ]) {
        input_events.send(InputEvent(IVec2::NEG_Y));
    }
    if input.any_just_released([
        KeyCode::KeyD,
        KeyCode::ArrowRight,
        KeyCode::KeyL,
        KeyCode::Numpad6,
    ]) {
        input_events.send(InputEvent(IVec2::X));
    }
    if input.any_just_released([
        KeyCode::KeyA,
        KeyCode::ArrowLeft,
        KeyCode::KeyH,
        KeyCode::Numpad4,
    ]) {
        input_events.send(InputEvent(IVec2::NEG_X));
    }
    if input.any_just_released([KeyCode::KeyY, KeyCode::Numpad7]) {
        input_events.send(InputEvent(IVec2::new(-1, 1)));
    }
    if input.any_just_released([KeyCode::KeyU, KeyCode::Numpad9]) {
        input_events.send(InputEvent(IVec2::new(1, 1)));
    }
    if input.any_just_released([KeyCode::KeyB, KeyCode::Numpad1]) {
        input_events.send(InputEvent(IVec2::new(-1, -1)));
    }
    if input.any_just_released([KeyCode::KeyN, KeyCode::Numpad3]) {
        input_events.send(InputEvent(IVec2::new(1, -1)));
    }
}

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_game_keyboard);

    }
}
