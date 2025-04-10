use deep_sea::{
    engine::Engine,
    error::DeepSeaResult,
    random_solver::RandomSolver,
    andrew::solver::AndrewGreedsAndRunsAwaySolver
};

fn run() -> DeepSeaResult {
    // let result = Engine::play_game();
    let result = Engine::evaluate_solvers::<(
        // AndrewTakesSolver,
        // AndrewTakesSolver,
        // AndrewTakesSolver,
        // AndrewTakesSolver,
        // AndrewTakesSolver,
        RandomSolver,
        RandomSolver,
        RandomSolver,
        RandomSolver,
        RandomSolver,
        AndrewGreedsAndRunsAwaySolver,
    )>(100_000)?;

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
