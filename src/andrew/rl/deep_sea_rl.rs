use crate::{
    solver::{TreasureDecision, TREASURE_DECISION_ENUM_COUNT},
    deep_sea::*,
    treasure::{Treasure, TREASURE_ENUM_COUNT, MAX_NUM_TREASURES}};
use bit_set::BitSet;
use itertools;
use ndarray::{Axis, Slice};


pub enum DeepSeaAction {
    TreasureDecision(TreasureDecision),
    DiveDirection(DiveDirection),
}


pub const DEEP_SEA_ACTION_ENUM_COUNT: usize =
    TREASURE_DECISION_ENUM_COUNT + DIVE_DIRECTION_ENUM_COUNT;


pub const PLAYER_DIM: usize = DIVE_DIRECTION_ENUM_COUNT
    + POSITION_ENUM_COUNT
    + MAX_NUM_TREASURES * TREASURE_ENUM_COUNT;


/* Deep-sea specific enum unpacking functions. */
pub fn treasure_enum_unpack<T>(treasure: &Treasure) -> T
where T: From<u32> {
    // update this to reflect real treasure point values?
    T::from(*treasure as u32)
}


pub fn dive_direction_enum_unpack<T>(dive_direction: &DiveDirection) -> T
where T: From<u32> {
    T::from(*dive_direction as u32)
}


pub fn position_enum_unpack<T>(position: &Position) -> Vec<T>
where T: From<u32> + From<bool> + From<usize>{
    let position_val: usize = match position {
        Position::Diving(depth) => *depth,
        _ => 0,
    };
    vec![T::from(position_val),
         T::from(*position == Position::WaitingToDive),
         T::from(*position == Position::ReturnedToSubmarine)]
}


pub fn tile_enum_unpack<T>(tile: &Tile) -> T
where T: From<u32> + num::Zero {
    match tile {
        Tile::Empty => T::zero(),
        Tile::Treasure(treasure) => treasure_enum_unpack(treasure),
    }
}


pub fn path_unpack<T>(path: &[Tile], occupied_tiles: &BitSet) -> Vec<T>
where T: From<u32> + From<bool> + num::Zero {
    let path_iter = path.iter().map(|tile| tile_enum_unpack(tile));
    let occupied_iter = (0..path.len()).map(|idx| T::from(occupied_tiles.contains(idx)));
    itertools::interleave(path_iter, occupied_iter).collect::<Vec<T>>()
}


pub fn player_unpack<T>(player: &Player) -> Vec<T>
where T: num::Zero + From<u32> + From<bool> + From<usize> {
    let direction = dive_direction_enum_unpack::<T>(&player.direction());
    let position = position_enum_unpack(&player.position());
    let held_treasures = player.held_treasures()
                               .iter()
                               .map(|t| treasure_enum_unpack::<T>(t));
    let mut unpacked = [direction].into_iter()
               .chain(position)
               .chain(held_treasures)
               .collect::<Vec<T>>();
    unpacked.resize_with(PLAYER_DIM, || T::zero());
    unpacked
}


pub fn action_unpack<T>(action: &DeepSeaAction) -> Vec<T>
where T: Clone + num::Zero + From<usize>{
    let mut unpacked = vec![T::zero(); DEEP_SEA_ACTION_ENUM_COUNT];
    let hot_idx = match action {
        DeepSeaAction::TreasureDecision(td) => match td {
            TreasureDecision::Ignore => 0,
            TreasureDecision::Take => 1,
            TreasureDecision::Return(t) => 2 + *t as usize,
        },
        DeepSeaAction::DiveDirection(dd) => TREASURE_DECISION_ENUM_COUNT + *dd as usize,
    };
    unpacked[hot_idx] = T::from(1);
    unpacked
}


pub fn raw_state_action_vector<T>(
    deep_sea: &DeepSea,
    deep_sea_action: &DeepSeaAction) -> ndarray::Array1<T>
where T: Clone + num::Zero + From<u32> + From<bool> + From<usize> {
    // Compute input dimension from state and action.
    let path_dim = deep_sea.path().len();
    let oxygen_dim: usize = 1;
    let occupied_tiles_dim = deep_sea.path().len();
    let action_dim = DEEP_SEA_ACTION_ENUM_COUNT;  // for one-hot vec
    let input_dim = path_dim + PLAYER_DIM + oxygen_dim + occupied_tiles_dim + action_dim;

    // Construct vector.
    let mut raw_vector: ndarray::Array1<T> = ndarray::Array1::<T>::zeros(input_dim);
    let mut offset = 0;
    // Path (with occupied tiles grouped in).
    let path_vec = ndarray::Array1::from(
        path_unpack(deep_sea.path(), deep_sea.occupied_tiles()));
    raw_vector.slice_axis_mut(
        Axis(0),
        Slice::from(offset..offset+path_dim+occupied_tiles_dim)
    ).assign(&path_vec);
    offset += path_dim + occupied_tiles_dim;
    // Players (treating player as 0th).
    let player_first_iter = deep_sea.players()
                                    .iter()
                                    .cycle()
                                    .skip(deep_sea.player_idx())
                                    .take(deep_sea.players().len());
    for player in player_first_iter {
        let player_vec = ndarray::Array1::from(player_unpack::<T>(player));
        raw_vector.slice_axis_mut(Axis(0), Slice::from(offset..offset+PLAYER_DIM))
                  .assign(&player_vec);
        offset += PLAYER_DIM;
    }
    // Oxygen.
    raw_vector[offset] = T::from(deep_sea.oxygen());
    offset += oxygen_dim;
    // Action.
    let action_vec = ndarray::Array1::from(action_unpack(deep_sea_action));
    raw_vector.slice_axis_mut(
        Axis(0),
        Slice::from(offset..offset+action_dim)
    ).assign(&action_vec);
    raw_vector
}
