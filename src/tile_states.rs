use bevy::{color::{Color, Srgba}, ecs::component::Component};

use crate::GameState;

#[derive(Component)]
pub struct Tile {
    pub x: usize,
    pub y: usize,
    pub tile: TileState,
    pub surrounding_mines: u8
}

impl Tile {
    fn get_color_hex (&self) -> &str {
        // Use xor for checker pattern
        let alt_color = (self.x % 2 == 0) ^ (self.y % 2 == 0);
        match self.tile {
            TileState::HiddenBlank | TileState::HiddenMine | TileState::FlaggedBlank | TileState::FlaggedMine => {
                if alt_color {"#a2d149"} else {"#aad751"}
            }
            TileState::RevealedBlank | TileState::RevealedNumber(_) => {
                if alt_color {"#d7b899"} else {"#e5c29f"}
            }
            TileState::RevealedMine => {"#ff0000"}
        }
    }

    pub fn get_color (&self) -> Color {
        if let Ok(srgba) = Srgba::hex(self.get_color_hex()) {Color::from(srgba)} else {Color::WHITE}
    }
}

#[derive(PartialEq, Clone, Copy)]
pub enum TileState {
    HiddenMine,
    HiddenBlank,
    RevealedMine,
    RevealedBlank,
    FlaggedMine,
    FlaggedBlank,
    RevealedNumber(u8)
}

impl TileState {
    // Return game state after reveal
    pub fn reveal (&mut self, surrounding_mines: u8) -> GameState {
        match self {
            TileState::HiddenMine => {
                *self = TileState::RevealedMine;
                GameState::Lose
            },
            TileState::HiddenBlank => {
                if surrounding_mines == 0 {
                    *self = TileState::RevealedBlank;
                    return GameState::Playing;
                }
                *self = TileState::RevealedNumber(surrounding_mines);
                GameState::Playing
            },
            _ => GameState::Playing
        }
    }

    pub fn is_revealed (self) -> bool {
        match self {
            TileState::RevealedMine | TileState::RevealedBlank | TileState::RevealedNumber(_) => true,
            _ => false
        }
    }

    pub fn is_mine (self) -> bool {
        match self {
            TileState::HiddenMine | TileState::FlaggedMine | TileState::RevealedMine => true,
            _ => false
        }
    }

    pub fn is_flagged (self) -> bool {
        match self {
            TileState::FlaggedMine | TileState::FlaggedBlank => true,
            _ => false
        }
    }
    
    pub fn toggle_flag (&mut self) {
        *self = match self {
            TileState::HiddenMine => TileState::FlaggedMine,
            TileState::HiddenBlank => TileState::FlaggedBlank,
            TileState::FlaggedMine => TileState::HiddenMine,
            TileState::FlaggedBlank => TileState::HiddenBlank,
            _ => *self
        }
    }
}