use bevy::prelude::*;

use crate::{GameState, GameValues, tile_states::Tile};

#[derive(EntityEvent, Clone)]
pub struct LeftClickTileEvent(pub Entity);

#[derive(EntityEvent, Clone)]
pub struct RightClickTileEvent(pub Entity);

pub fn click () -> impl Fn(On<Pointer<Press>>, Commands, ResMut<GameValues>) {
    move |ev: On<'_, '_, Pointer<Press>>, mut commands, game_values| {
        if game_values.state != GameState::Playing {return}
        match ev.button {
            PointerButton::Primary => {commands.entity(ev.entity).trigger(LeftClickTileEvent);},
            PointerButton::Secondary => {commands.entity(ev.entity).trigger(RightClickTileEvent);},
            _ => return,
        }
    }
}

pub fn left_click () -> impl Fn(On<LeftClickTileEvent>, Commands, Query<(&mut Tile, &mut MeshMaterial2d<ColorMaterial>, &Children, &mut Transform)>, ResMut<Assets<ColorMaterial>>, ResMut<GameValues>) {
    move |ev: On<'_, '_, LeftClickTileEvent>, mut commands, mut info, mut color_materials, mut game_values| {   
        let Ok((mut tile, material, children, mut transform)) = info.get_mut(ev.event_target()) else {
            return;
        };
        let Some(mut color) = color_materials.get_mut(&material.0) else {
            return;
        };
        let surrounding_mines = tile.surrounding_mines;
        if !tile.tile.is_revealed() && !tile.tile.is_flagged() {
            match game_values.state {
                GameState::Lose | GameState::End => {tile.tile.reveal(surrounding_mines);}
                _ => {game_values.state = tile.tile.reveal(surrounding_mines);}
            }
            if !tile.tile.is_mine() {game_values.remaining_blanks -= 1}
            transform.translation.z = -1.0;
            if surrounding_mines != 0 {
                if !tile.tile.is_mine() {
                    commands.entity(*children.get(1).unwrap()).insert(Visibility::Visible);
                }
            }
            else {
                for dx in -1isize..=1 {
                    for dy in -1isize..=1 {
                        let x = tile.x as isize + dx;
                        let y = tile.y as isize + dy;
                        if x < 0 || y < 0 || (dx == 0 && dy == 0) {continue}
                        let Some(row) = game_values.tile_entities.get(x as usize) else {continue};
                        let Some(spread_tile) = row.get(y as usize) else {continue};
                        let entity = *spread_tile;
                        commands.entity(entity).trigger(LeftClickTileEvent);
                    }
                }
            }
        }
        color.color = tile.get_color();
    }
}

pub fn right_click () -> impl Fn(On<RightClickTileEvent>, Commands, Query<(&mut Tile, &Children)>) {
    move |ev: On<'_, '_, RightClickTileEvent>, mut commands, mut info| {   
        let Ok((mut tile, children)) = info.get_mut(ev.event_target()) else {
            return;
        };
        tile.tile.toggle_flag();
        commands.entity(*children.get(0).unwrap()).insert(match tile.tile.is_flagged() {
            true => Visibility::Visible,
            false => Visibility::Hidden
        });
    }
}