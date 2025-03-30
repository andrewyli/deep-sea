use crate::deep_sea::{DeepSea, DiveDirection};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum TreasureDecision {
    Ignore,
    Take,
    Return,
}

pub trait DeepSeaSolver: Default {
    fn choose_direction(&mut self, deep_sea: &DeepSea, player_idx: usize) -> DiveDirection;

    fn take_treasure(&mut self, deep_sea: &DeepSea, player_idx: usize) -> TreasureDecision;
}
