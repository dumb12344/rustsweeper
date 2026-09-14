use bevy::prelude::*;
use rand::Rng;

use crate::{GameState, GameValues, constants::{MINE_COUNT, NUMBER_COLORS, SIZE_X, SIZE_Y}, tile_states::{Tile, TileState}};

#[derive(EntityEvent, Clone)]
pub struct ResetTileEvent(pub Entity);

#[derive(Event, Clone)]
pub struct ResetGameEvent();

pub fn reset () -> impl Fn(On<ResetTileEvent>, Commands, Query<(&mut Tile, &mut MeshMaterial2d<ColorMaterial>, &Children, &mut Transform)>, ResMut<Assets<ColorMaterial>>) {
    move |ev: On<'_, '_, ResetTileEvent>, mut commands, mut info, mut color_materials| {
        let Ok((tile, material, children, mut transform)) = info.get_mut(ev.event_target()) else {return};
        let Some(mut color) = color_materials.get_mut(&material.0) else {return};
        let mut number_entity = commands.entity(*children.get(1).unwrap());
        number_entity.insert(Text2d::new(tile.surrounding_mines.to_string()));
        number_entity.insert(TextColor(Color::from(Srgba::hex(NUMBER_COLORS.get(tile.surrounding_mines as usize).unwrap()).unwrap())));
        
        // Flag, number, and hover effect
        [0, 1, 3].iter().for_each(|i| {
            commands.entity(*children.get(*i).unwrap()).insert(Visibility::Hidden);
        });
        transform.translation.z = 0.0;
        color.color = tile.get_color();
    }
}

pub fn reset_game (_: On<ResetGameEvent>, mut commands: Commands, mut game_values: ResMut<GameValues>, mut tile_query: Query<&mut Tile>){
    game_values.state = GameState::Initializing;
    game_values.remaining_blanks = (SIZE_X as i32 * SIZE_Y as i32) - MINE_COUNT as i32;
    game_values.tile_entities.iter_mut().for_each(|row| {
        row.iter_mut().for_each(|tile_entity| {
            commands.entity(*tile_entity).trigger(ResetTileEvent);
            tile_query.get_mut(*tile_entity).unwrap().tile = TileState::HiddenBlank;
        });
    });
    generate_mines(&mut game_values.tile_entities, &mut tile_query);
    generate_surrounding_mines(&mut game_values.tile_entities, &mut tile_query);
    if let Some(end_screen_entity) = game_values.end_screen_entity {
        commands.entity(end_screen_entity).despawn();
        game_values.end_screen_entity = None;
    };
    game_values.state = GameState::Playing;
}

pub fn generate_surrounding_mines (tiles: &mut Vec<Vec<Entity>>, tile_query: &mut Query<&mut Tile>) {
    tiles.iter().enumerate().for_each(|(i, tile_row)| {
        tile_row.iter().enumerate().for_each(|(j, entity)| {
            tile_query.get_mut(*entity).unwrap().surrounding_mines = 0;
            for dx in -1isize..=1 {
                for dy in -1isize..=1 {
                    let x = i as isize + dx;
                    let y = j as isize + dy;
                    if x < 0 || y < 0 {continue};
                    let Some(row) = tiles.get(x as usize) else {continue};
                    let Some(loop_tile) = row.get(y as usize) else {continue};
                    let Ok(tile_entity) = tile_query.get_mut(*loop_tile) else {continue};
                    if tile_entity.tile.is_mine() {tile_query.get_mut(*entity).unwrap().surrounding_mines += 1};
                }
            }
        });
    });
}

pub fn generate_mines (tiles: &mut Vec<Vec<Entity>>, tile_query: &mut Query<&mut Tile>) {
    let mut i = 0;
    while i < MINE_COUNT as i32 {
        let rand_x = rand::thread_rng().gen_range(0..SIZE_X) as usize;
        let rand_y = rand::thread_rng().gen_range(0..SIZE_Y) as usize;
        // Don't place mine in top left corner
        let Ok(mut tile) = tile_query.get_mut(tiles[rand_x][rand_y]) else {continue};
        if tile.tile == TileState::HiddenBlank && !(rand_x <= 1 && rand_y >= SIZE_Y as usize - 2) {
            tile.tile = TileState::HiddenMine;
            i += 1;
        }
    }
}