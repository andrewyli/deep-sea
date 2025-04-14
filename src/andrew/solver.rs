use rand::Rng;

use crate::{
    deep_sea::{DeepSea, DiveDirection, Position, Tile},
    solver::{DeepSeaSolver, TreasureDecision},
    treasure::Treasure,
    andrew::utils::{Dice, Die},
};

#[derive(Default)]
pub struct AndrewTakesSolver;

impl DeepSeaSolver for AndrewTakesSolver {
    fn choose_direction(&mut self, deep_sea: &DeepSea, player_idx: usize) -> DiveDirection {
        let player = &deep_sea.players()[player_idx];
        if player.position() == Position::WaitingToDive || rand::rng().random_bool(0.5) {
            DiveDirection::Down
        } else {
            DiveDirection::Up
        }
    }

    fn take_treasure(&mut self, deep_sea: &DeepSea, player_idx: usize) -> TreasureDecision {
        let player = &deep_sea.players()[player_idx];
        let pos = match player.position() {
            Position::Diving(x) => x,
            _ => panic!("we're not diving"),
        };
        match deep_sea.path()[pos] {
            Tile::Treasure(Treasure::One) |
            Tile::Treasure(Treasure::Two) |
            Tile::Treasure(Treasure::Three) |
            Tile::Treasure(Treasure::Four) => TreasureDecision::Take,
            _ => TreasureDecision::Ignore,
        }
    }
}

#[derive(Default)]
pub struct AndrewGreedsAndRunsAwaySolver {
    returning: bool
}

impl AndrewGreedsAndRunsAwaySolver {

    fn get_estimated_rolls_to_return(&self, deep_sea: &DeepSea, player_idx: usize, with_take: Option<bool>) -> u32 {
        let with_take = with_take.unwrap_or(false);
        let dice = Dice {
            dice: vec![
                Die::from_vec(vec![1, 1, 2, 2, 3, 3]),
                Die::from_vec(vec![1, 1, 2, 2, 3, 3]),
            ]
        };
        let player = &deep_sea.players()[player_idx];
        let penalty = player.held_treasures().len() + (with_take as usize);
        match deep_sea.players()[player_idx].position() {
            Position::Diving(depth) => (depth as f32 / (dice.mean() - penalty as f32)) as u32,
            _ => 0,
        }
    }

    fn get_estimated_remaining_turns(&self, deep_sea: &DeepSea, _player_idx: usize) -> u32 {
        let mut current_tiles: u32 = deep_sea.players().iter().map(|p| p.held_treasures().len()).sum::<usize>() as u32;
        let mut oxygen: u32 = deep_sea.oxygen();
        let mut turns_remaining = 0u32;
        while oxygen > 0 {
            oxygen = oxygen.saturating_sub(current_tiles);
            current_tiles += deep_sea.players().len() as u32;
            turns_remaining += 1;
        }
        turns_remaining
    }
}

impl DeepSeaSolver for AndrewGreedsAndRunsAwaySolver {

    fn choose_direction(&mut self, deep_sea: &DeepSea, player_idx: usize) -> DiveDirection {
        if self.returning {
            return DiveDirection::Up;
        }
        let player = &deep_sea.players()[player_idx];
        let est_turns_left = self.get_estimated_remaining_turns(
            deep_sea, player_idx);
        let est_rolls_to_return = self.get_estimated_rolls_to_return(
            deep_sea, player_idx, None);
        if player.position() == Position::WaitingToDive || est_turns_left <= est_rolls_to_return {
            DiveDirection::Down
        } else {
            self.returning = true;
            DiveDirection::Up
        }
    }

    fn take_treasure(&mut self, deep_sea: &DeepSea, player_idx: usize) -> TreasureDecision {
        if self.returning {
            let est_turns_left = self.get_estimated_remaining_turns(
                deep_sea, player_idx);
            let est_rolls_to_return = self.get_estimated_rolls_to_return(
                deep_sea, player_idx, None);
            if est_turns_left <= est_rolls_to_return {
                return TreasureDecision::Ignore;
            }
        }
        let player = &deep_sea.players()[player_idx];
        let pos = match player.position() {
            Position::Diving(x) => x,
            _ => panic!("we're not diving"),
        };
        match deep_sea.path()[pos] {
            Tile::Treasure(Treasure::One) |
            Tile::Treasure(Treasure::Two) |
            Tile::Treasure(Treasure::Three) |
            Tile::Treasure(Treasure::Four) => TreasureDecision::Take,
            _ => TreasureDecision::Ignore,
        }
    }
}

#[derive(Default)]
pub struct AndrewIsPatientAndRunsAwaySolver {
    returning: bool
}

impl AndrewIsPatientAndRunsAwaySolver {

    fn get_estimated_rolls_to_return(&self, deep_sea: &DeepSea, player_idx: usize, with_take: bool) -> u32 {
        let dice = Dice {
            dice: vec![
                Die::from_vec(vec![1, 1, 2, 2, 3, 3]),
                Die::from_vec(vec![1, 1, 2, 2, 3, 3]),
            ]
        };
        let player = &deep_sea.players()[player_idx];
        let penalty = player.held_treasures().len() + (with_take as usize);
        match deep_sea.players()[player_idx].position() {
            Position::Diving(depth) => (depth as f32 / (dice.mean() - penalty as f32)) as u32,
            _ => 0,
        }
    }

    fn get_estimated_remaining_turns(&self, deep_sea: &DeepSea, _player_idx: usize) -> u32 {
        let mut current_tiles: u32 = deep_sea.players().iter().map(|p| p.held_treasures().len()).sum::<usize>() as u32;
        let mut oxygen: u32 = deep_sea.oxygen();
        let mut turns_remaining = 0u32;
        while oxygen > 0 {
            oxygen = oxygen.saturating_sub(current_tiles);
            current_tiles += deep_sea.players().len() as u32;
            turns_remaining += 1;
        }
        turns_remaining
    }
}

impl DeepSeaSolver for AndrewIsPatientAndRunsAwaySolver {

    fn choose_direction(&mut self, deep_sea: &DeepSea, player_idx: usize) -> DiveDirection {
        if self.returning {
            return DiveDirection::Up;
        }
        let player = &deep_sea.players()[player_idx];
        let est_turns_left = self.get_estimated_remaining_turns(
            deep_sea, player_idx);
        let est_rolls_to_return = self.get_estimated_rolls_to_return(
            deep_sea, player_idx, false);
        if player.position() == Position::WaitingToDive || est_turns_left <= est_rolls_to_return {
            DiveDirection::Down
        } else {
            self.returning = true;
            DiveDirection::Up
        }
    }

    fn take_treasure(&mut self, deep_sea: &DeepSea, player_idx: usize) -> TreasureDecision {
        let est_turns_left = self.get_estimated_remaining_turns(
            deep_sea, player_idx);
        let est_rolls_to_return = self.get_estimated_rolls_to_return(
            deep_sea, player_idx, false);
        if est_turns_left <= est_rolls_to_return {
            return TreasureDecision::Ignore;
        }
        let player = &deep_sea.players()[player_idx];
        let pos = match player.position() {
            Position::Diving(x) => x,
            _ => panic!("we're not diving"),
        };
        match deep_sea.path()[pos] {
            Tile::Treasure(Treasure::One) |
            Tile::Treasure(Treasure::Two) |
            Tile::Treasure(Treasure::Three) |
            Tile::Treasure(Treasure::Four) => TreasureDecision::Take,
            _ => TreasureDecision::Ignore,
        }
    }
}
