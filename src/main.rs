use deep_sea::{engine::Engine, error::DeepSeaResult};

fn main() -> DeepSeaResult {
    let result = Engine::play_game();
    if let Err(err) = &result {
        eprintln!("{err}");
    }
    result
}
