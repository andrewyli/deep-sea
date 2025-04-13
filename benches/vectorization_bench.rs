use criterion::{black_box, criterion_group, criterion_main, Criterion};

use deep_sea::{
    deep_sea::{DeepSea, DiveDirection, Tile},
    treasure::Treasure,
    ml::vectorization::*,
    deep_sea_vectorization::{DeepSeaAction, DeepSeaState, DeepSeaStateActionPair, Path, Player}};
use bit_set::BitSet;


fn get_default_state() -> DeepSeaState {
    let tiles: Vec<Tile> = (0..8)
        .map(|_| Tile::Treasure(Treasure::One))
        .chain((0..8).map(|_| Tile::Treasure(Treasure::Two)))
        .chain((0..8).map(|_| Tile::Treasure(Treasure::Three)))
        .chain((0..8).map(|_| Tile::Treasure(Treasure::Four)))
        .collect();
    let path = Path { tiles, occupied: BitSet::new() };
    DeepSeaState {
        path,
        players: vec![Player::new(); 6],
        oxygen: DeepSea::OXYGEN as u16,
    }
}


fn bench_state_action_vectorization(c: &mut Criterion) {
    let default_state = get_default_state();
    c.bench_function(
        "starting state vectorization",
        |b| b.iter_batched(
            || black_box(DeepSeaStateActionPair {
                state: &default_state,
                action: &DeepSeaAction::DiveDirection(DiveDirection::Down),}),
            |data| data.into_ndarray::<f32>(),
            criterion::BatchSize::SmallInput
        )
    );
}


fn bench_state_action_unpacking(c: &mut Criterion) {
    let default_state = get_default_state();
    let state_action = black_box(DeepSeaStateActionPair {
        state: &default_state,
        action: &DeepSeaAction::DiveDirection(DiveDirection::Down),
    });
    c.bench_function(
        "starting state unpacking",
        |b| b.iter(|| state_action.unpack::<f32>()),
    );
}


criterion_group!(
    benches,
    bench_state_action_unpacking,
    bench_state_action_vectorization
);
criterion_main!(benches);
