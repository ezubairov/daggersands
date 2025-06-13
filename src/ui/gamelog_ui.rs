use bevy::{
    color::palettes::css::WHITE,
    input::mouse::{MouseScrollUnit, MouseWheel},
    picking::hover::HoverMap,
    render::view::RenderLayers,
    text::LineBreak,
};

use crate::{camera::GamelogCamera, prelude::*};

#[derive(Component)]
pub struct GamelogUIRoot;

#[derive(Default, Resource)]
pub struct Gamelog {
    entries: Vec<String>,
    dirty: bool,
}

impl Gamelog {
    fn add(&mut self, string: String) {
        self.entries.push(string);
        self.dirty = true
    }
}

const LINE_HEIGHT: f32 = 21.;
const FONT_SIZE: f32 = 14.;

fn setup_gamelog_ui(camera: Query<Entity, With<GamelogCamera>>, mut commands: Commands) {
    commands
        .spawn((
            GamelogUIRoot,
            Node {
                width: Val::Vw(100.),
                height: Val::Vh(100.),
                padding: UiRect::all(Val::Px(3.)),
                flex_direction: FlexDirection::Column,
                ..default()
            },
            RenderLayers::layer(28),
            UiTargetCamera(camera.single().unwrap()),
        ))
        .insert(Pickable::IGNORE);

    commands.insert_resource(Gamelog {
        entries: vec![],
        dirty: true,
    });
}

fn render_gamelog(
    mut commands: Commands,
    gamelog_root: Query<Entity, With<GamelogUIRoot>>,
    mut gamelog: ResMut<Gamelog>,
) {
    if gamelog.dirty {
        commands
            .entity(gamelog_root.single().unwrap())
            .with_children(|parent| {
                parent
                    .spawn((
                        Node {
                            flex_direction: FlexDirection::Column,
                            align_self: AlignSelf::Stretch,
                            overflow: Overflow::scroll_y(),
                            border: UiRect::all(Val::Px(3.)),
                            flex_grow: 1.,
                            ..default()
                        },
                        BorderColor(WHITE.into()),
                    ))
                    .with_children(|parent| {
                        for (j, i) in gamelog.entries.iter().enumerate() {
                            parent
                                .spawn(Node {
                                    min_height: Val::Px(LINE_HEIGHT),
                                    max_height: Val::Px(LINE_HEIGHT),
                                    ..default()
                                })
                                .insert(Pickable {
                                    should_block_lower: false,
                                    ..default()
                                })
                                .with_children(|parent| {
                                    parent
                                        .spawn((
                                            Text::new(format!("{j}{i}\n")),
                                            TextFont {
                                                font_size: FONT_SIZE,
                                                ..default()
                                            },
                                            TextLayout::new(
                                                JustifyText::Left,
                                                LineBreak::WordOrCharacter,
                                            ),
                                            Label,
                                        ))
                                        .insert(Pickable {
                                            should_block_lower: false,
                                            ..default()
                                        });
                                });
                        }
                    });
            });

        gamelog.entries = vec![];
        gamelog.dirty = false
    }
}

fn update_scroll_position(
    mut mouse_wheel_events: EventReader<MouseWheel>,
    hover_map: Res<HoverMap>,
    mut scrolled_node_query: Query<&mut ScrollPosition>,
) {
    for mouse_wheel_event in mouse_wheel_events.read() {
        let (dx, dy) = match mouse_wheel_event.unit {
            MouseScrollUnit::Line => (
                mouse_wheel_event.x * LINE_HEIGHT,
                mouse_wheel_event.y * LINE_HEIGHT,
            ),
            MouseScrollUnit::Pixel => (mouse_wheel_event.x, mouse_wheel_event.y),
        };

        for (_pointer, pointer_map) in hover_map.iter() {
            for (entity, _hit) in pointer_map.iter() {
                if let Ok(mut scroll_position) = scrolled_node_query.get_mut(*entity) {
                    scroll_position.offset_x -= dx;
                    scroll_position.offset_y -= dy;
                }
            }
        }
    }
}

pub struct GamelogUIPlugin;
impl Plugin for GamelogUIPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Gamelog>()
            // PostStartup as we need cameras to be already set up, but that's a hack
            // TODO: look for a way of defining dependency/order on a different plugin
            .add_systems(PostStartup, setup_gamelog_ui)
            .add_systems(Update, render_gamelog)
            .add_systems(Update, update_scroll_position);
    }
}
