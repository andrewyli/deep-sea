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

#[derive(Clone, Debug)]
pub struct Player {
    direction: DiveDirection,
    tile: Position,
    held_treasures: Vec<Treasure>,
}

impl Player {
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
    oxygen: u32,
}

impl DeepSea {
    pub fn path(&self) -> &[Tile] {
        &self.path
    }

    pub fn players(&self) -> &[Player] {
        &self.players
    }

    pub fn oxygen(&self) -> u32 {
        self.oxygen
    }
}
