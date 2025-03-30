use deep_sea::{engine::Engine, error::DeepSeaResult, random_solver::RandomSolver};

fn run() -> DeepSeaResult {
    // let result = Engine::play_game();
    let result = Engine::evaluate_solvers::<(
        RandomSolver,
        RandomSolver,
        RandomSolver,
        RandomSolver,
        RandomSolver,
        RandomSolver,
    )>(1_000_000)?;

    println!("Result: {result:?}");

    Ok(())
}

fn main() -> DeepSeaResult {
    let result = run();
    if let Err(err) = &result {
        eprintln!("{err}");
    }
    result
}
