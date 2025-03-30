use bit_set::BitSet;
use itertools::Itertools;

use crate::{
    error::{DeepSeaError, DeepSeaResult},
    solver::TreasureDecision,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Treasure {
    One,
    Two,
    Three,
    Four,
}

impl Treasure {}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Tile {
    Empty,
    Treasure(Treasure),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum DiveDirection {
    Down,
    Up,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Position {
    /// Index into `DeepSea::path`.
    Diving(usize),
    WaitingToDive,
    ReturnedToSubmarine,
}

impl Position {
    pub fn as_diving(&self) -> Option<usize> {
        if let Self::Diving(index) = self {
            Some(*index)
        } else {
            None
        }
    }

    pub fn advance(&self, direction: DiveDirection) -> Position {
        match (self, direction) {
            (Self::Diving(0), DiveDirection::Up) => Self::ReturnedToSubmarine,
            (Self::Diving(index), DiveDirection::Up) => Self::Diving(index - 1),
            (Self::Diving(index), DiveDirection::Down) => Self::Diving(index + 1),
            (Self::WaitingToDive, DiveDirection::Down) => Self::Diving(0),
            (Self::WaitingToDive, DiveDirection::Up) => {
                unreachable!("Cannot move up before leaving submarine")
            }
            (Self::ReturnedToSubmarine, _) => {
                unreachable!("Cannot move after returned to submarine")
            }
        }
    }
}

#[derive(Clone, Debug)]
pub struct Player {
    direction: DiveDirection,
    tile: Position,
    held_treasures: Vec<Treasure>,
}

impl Player {
    fn new() -> Self {
        Self {
            direction: DiveDirection::Down,
            tile: Position::WaitingToDive,
            held_treasures: vec![],
        }
    }

    pub fn direction(&self) -> DiveDirection {
        self.direction
    }

    pub fn tile(&self) -> Position {
        self.tile
    }

    pub fn held_treasures(&self) -> &[Treasure] {
        &self.held_treasures
    }
}

#[derive(Clone, Debug)]
pub struct DeepSea {
    path: Vec<Tile>,
    players: Vec<Player>,
    player_idx: usize,
    oxygen: u32,
    occupied_tiles: BitSet,
}

impl DeepSea {
    pub const OXYGEN: u32 = 25;

    pub fn new(path: Vec<Tile>, num_players: usize) -> Self {
        let path_len = path.len();
        Self {
            path,
            players: (0..num_players).map(|_| Player::new()).collect(),
            player_idx: 0,
            oxygen: Self::OXYGEN,
            occupied_tiles: BitSet::with_capacity(path_len),
        }
    }

    pub fn path(&self) -> &[Tile] {
        &self.path
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn player_idx(&self) -> usize {
        self.player_idx
    }

    pub fn oxygen(&self) -> u32 {
        self.oxygen
    }

    fn at_end(&self, position: Position) -> bool {
        position == Position::Diving(self.path.len() - 1)
    }

    fn occupied(&self, position: Position) -> bool {
        match position {
            Position::Diving(index) => self.occupied_tiles.contains(index),
            Position::ReturnedToSubmarine | Position::WaitingToDive => false,
        }
    }

    fn leave_tile(&mut self, position: Position) {
        match position {
            Position::Diving(index) => {
                self.occupied_tiles.remove(index);
            }
            Position::WaitingToDive | Position::ReturnedToSubmarine => {}
        }
    }

    fn enter_tile(&mut self, position: Position) {
        match position {
            Position::Diving(index) => {
                self.occupied_tiles.insert(index);
            }
            Position::WaitingToDive | Position::ReturnedToSubmarine => {}
        }
    }

    pub fn move_player(&mut self, direction: DiveDirection, mut dice_roll: u32) {
        let player = &self.players[self.player_idx];
        dice_roll = dice_roll.saturating_sub(player.held_treasures().len() as u32);

        let mut cur_player_pos = player.tile();
        let mut player_pos = cur_player_pos;
        while dice_roll > 0 {
            if direction == DiveDirection::Down && self.at_end(player_pos) {
                break;
            }

            cur_player_pos = cur_player_pos.advance(direction);
            if !self.occupied(cur_player_pos) {
                dice_roll -= 1;
                player_pos = cur_player_pos;
            }
        }

        self.leave_tile(player.tile());
        self.enter_tile(player_pos);
        let player = &mut self.players[self.player_idx];
        player.direction = direction;
        player.tile = player_pos;
    }

    pub fn take_treasure(&mut self, treasure: TreasureDecision) -> DeepSeaResult {
        let player = &mut self.players[self.player_idx];
        let tile_idx = player.tile().as_diving().unwrap();
        match treasure {
            TreasureDecision::Take => {
                if let Tile::Treasure(treasure) = self.path[tile_idx] {
                    player.held_treasures.push(treasure);
                    Ok(())
                } else {
                    Err(DeepSeaError::AgentError(format!(
                        "Cannot take treasure from {tile_idx}: {:?}",
                        self.path[tile_idx]
                    ))
                    .into())
                }
            }
            TreasureDecision::Return(treasure) => {
                if self.path[tile_idx] != Tile::Empty {
                    Err(DeepSeaError::AgentError(format!(
                        "Cannot put treasure back in non-empty tile {tile_idx}: {:?}",
                        self.path[tile_idx]
                    ))
                    .into())
                } else if let Some((treasure_idx, _)) = player
                    .held_treasures
                    .iter()
                    .find_position(|t| **t == treasure)
                {
                    player.held_treasures.remove(treasure_idx);
                    Ok(())
                } else {
                    Err(DeepSeaError::AgentError(format!(
                        "Player is not holding treasure {:?}: {:?}",
                        treasure, player.held_treasures
                    ))
                    .into())
                }
            }
            TreasureDecision::Ignore => Ok(()),
        }
    }

    pub fn next_player(&mut self) {
        self.player_idx = (self.player_idx + 1) % self.players.len();
    }
}
