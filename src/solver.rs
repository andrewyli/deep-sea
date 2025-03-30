pub enum Move {
    Down,
    Up,
}

pub enum TreasureDecision {
    Ignore,
    Take,
    Return,
}

pub trait DeepSeaSolver {
    fn choose_direction() -> Move;

    fn take_treasure() -> TreasureDecision;
}
