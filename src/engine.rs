use rand::Rng;

use crate::{
    deep_sea::{DeepSea, DiveDirection, Position},
    error::DeepSeaResult,
    solver::DeepSeaSolver,
};

pub struct Engine {
    state: DeepSea,
    players: Vec<Box<dyn DeepSeaSolver>>,
}

impl Engine {
    fn roll_dice() -> u32 {
        let mut rng = rand::rng();
        let d1 = rng.random_range(1..=3);
        let d2 = rng.random_range(1..=3);
        d1 + d2
    }

    fn take_turn(&mut self) -> DeepSeaResult {
        let player_idx = self.state.player_idx();
        let player = &self.state.players()[player_idx];
        let player_agent = &mut self.players[player_idx];

        let direction = if player.direction() == DiveDirection::Down {
            player_agent.choose_direction(&self.state, player_idx)
        } else {
            DiveDirection::Up
        };

        let dice_roll = Self::roll_dice();
        self.state.move_player(direction, dice_roll);

        let player = &self.state.players()[player_idx];
        if let Position::Diving(_) = player.tile() {
            self.state
                .take_treasure(player_agent.take_treasure(&self.state, player_idx))?;
        }

        self.state.next_player();
        Ok(())
    }
}
