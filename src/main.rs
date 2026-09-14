use std::cmp;

use bevy::prelude::*;
use bevy::sprite::Text2dShadow;
#[cfg(feature = "embed-assets")]
use bevy_embedded_assets::{EmbeddedAssetPlugin, PluginMode};

use crate::{pointer_handling::RevealTileEvent, constants::*, reset::ResetGameEvent, tile_states::Tile, init::setup};
mod tile_states;
mod pointer_handling;
mod init;
mod constants;
mod reset;

fn main () {
    let mut app = App::new();
    #[cfg(feature = "embed-assets")]
    app.add_plugins(EmbeddedAssetPlugin {mode: PluginMode::ReplaceDefault});
    app.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(Window {
            title: "Rustsweeper".to_string(),
            mode: bevy::window::WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
            ..Default::default()
        }),
        ..Default::default()
    }));
    app.add_plugins(MeshPickingPlugin);
    app.init_resource::<GameValues>();
    app.add_systems(Startup, set_width);
    app.add_systems(Startup, setup);
    app.add_systems(Update, win_check);
    app.add_systems(Update, manage_keys);
    app.run();
}

#[derive(Default, PartialEq)]
pub enum GameState {
    #[default]
    Initializing,
    Playing,
    Win,
    Lose,
    End
}

#[derive(Resource, Default)]
pub struct GameValues {
    pub state: GameState,
    pub remaining_blanks: i32,
    pub tile_entities: Vec<Vec<Entity>>,
    pub tile_size: u32,
    pub end_screen_entity: Option<Entity>
}

fn max (a: u32, b: u32) -> u32 {
    if a > b {
        a
    }
    else {
        b
    }
}

fn compare (a: f32, b: f32) -> cmp::Ordering {
    if a > b {
        cmp::Ordering::Greater
    }
    else if a < b {
        cmp::Ordering::Less
    }
    else {
        cmp::Ordering::Equal
    }
}

fn set_width (windows: Query<&mut Window>, mut game_values: ResMut<GameValues>) {
    let Ok(window) = windows.single() else {game_values.tile_size = 50;return};
    game_values.tile_size = (match compare(window.width(), window.height()) {
        cmp::Ordering::Greater => {window.height() / SIZE_Y as f32}
        cmp::Ordering::Less => {window.width() / SIZE_X as f32},
        cmp::Ordering::Equal => {window.width() / max(SIZE_X, SIZE_Y) as f32},
    } as u32);
}

fn manage_keys (mut game_values: ResMut<GameValues>, mut commands: Commands, tile_query: Query<&mut Tile>, button: Res<ButtonInput<KeyCode>>) {
    if button.just_pressed(KeyCode::KeyM) {
        game_values.tile_entities.iter_mut().enumerate().for_each(|(i, row)| {
            row.iter_mut().enumerate().for_each(|(j, tile_entity)| {
                if !tile_query.get(*tile_entity).unwrap().tile.is_mine() {
                    commands.delayed().secs((i as u32 + SIZE_X * j as u32) as f32 / (SIZE_X * SIZE_Y) as f32).entity(*tile_entity).trigger(RevealTileEvent);
                }
            });
        });
    }
    if button.just_pressed(KeyCode::KeyR) {
        commands.trigger(ResetGameEvent{});
    }
}

fn win_check (mut game_values: ResMut<GameValues>, mut commands: Commands, tile_query: Query<&mut Tile>) {
    game_values.state = match game_values.state {
        GameState::Lose => {
            game_values.end_screen_entity = Some(commands.spawn((
                Text2d::new("YOU LOSE"),
                Transform::from_xyz(0.0,0.0, 3.0),
                TextFont {
                    font_size: FontSize::Px(40.0 / 50.0 * game_values.tile_size as f32),
                    weight: FontWeight(30),
                    ..Default::default()
                },
                TextColor(Color::from(Srgba::hex("#ff0000").unwrap())),
                Text2dShadow::default()
            )).id());
            game_values.tile_entities.iter_mut().for_each(|row| {
                row.iter_mut().for_each(|tile_entity| {
                    if tile_query.get(*tile_entity).unwrap().tile.is_mine() {
                        commands.entity(*tile_entity).trigger(RevealTileEvent);
                    }
                });
            });
            GameState::End
        },
        GameState::Playing => if game_values.remaining_blanks <= 0 {GameState::Win} else {GameState::Playing},
        GameState::Win => {
            game_values.end_screen_entity = Some(commands.spawn((
                Text2d::new("YOU WIN!!"),
                Transform::from_xyz(0.0,0.0, 3.0),
                TextFont {
                    font_size: FontSize::Px(40.0 / 50.0 * game_values.tile_size as f32),
                    weight: FontWeight(30),
                    ..Default::default()
                },
                TextColor(Color::from(Srgba::hex("#9c5300").unwrap())),
                Text2dShadow::default()
            )).id());
            GameState::End
        },
        GameState::End => GameState::End,
        GameState::Initializing => GameState::Initializing,
    }
}