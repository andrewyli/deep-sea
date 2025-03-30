use rand::Rng;

use crate::{
    deep_sea::{DeepSea, DiveDirection, Tile},
    solver::{DeepSeaSolver, TreasureDecision},
};

#[derive(Default)]
pub struct RandomSolver;

impl DeepSeaSolver for RandomSolver {
    fn choose_direction(&mut self, deep_sea: &DeepSea, player_idx: usize) -> DiveDirection {
        debug_assert_eq!(
            deep_sea.players()[player_idx].direction(),
            DiveDirection::Down
        );

        if rand::rng().random_bool(0.5) {
            DiveDirection::Down
        } else {
            DiveDirection::Up
        }
    }

    fn take_treasure(&mut self, deep_sea: &DeepSea, player_idx: usize) -> TreasureDecision {
        let player = &deep_sea.players()[player_idx];
        let tile = deep_sea.path()[player.tile().as_diving().unwrap()];
        match tile {
            Tile::Empty => {
                if player.held_treasures().is_empty() || rand::rng().random_bool(0.5) {
                    TreasureDecision::Ignore
                } else {
                    TreasureDecision::Return
                }
            }
            Tile::Treasure(_) => {
                if rand::rng().random_bool(0.5) {
                    TreasureDecision::Take
                } else {
                    TreasureDecision::Ignore
                }
            }
        }
    }
}
