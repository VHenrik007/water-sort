use std::time::Instant;
use thiserror::Error;

use water_sort::{
    cli_arguments::{Args, ArgumentsError, SearchMethod},
    solver::{bfs_shortest_path, heuristic_dijkstra_search, solve, SolverError},
};

/// Custom error for the solver.
#[derive(Debug, Error)]
pub enum WaterSortError {
    /// Derived from the SolverError if an issue happens on that level.
    #[error(transparent)]
    SolverError(#[from] SolverError),
    /// Derived from the ArgumentsError if an issue happens on that level.
    #[error(transparent)]
    ArgumentsError(#[from] ArgumentsError),
}

pub type WaterSortResult<T> = Result<T, WaterSortError>;

fn main() -> WaterSortResult<()> {
    let (args, system_to_solve) = Args::process_arguments()?;

    system_to_solve.print_system_state();

    println!("Solving...");

    let now = Instant::now();
    let solution_steps = match args.search_method {
        SearchMethod::Bfs => bfs_shortest_path(&system_to_solve),
        SearchMethod::Heuristic => {
            heuristic_dijkstra_search(&system_to_solve, &args.heuristic_evaluation.into())
        }
    }?;
    let elapsed = now.elapsed();

    let solved_system = solve(system_to_solve, &solution_steps)?;

    if solved_system.is_solved() {
        println!(
            "Found a {} step solution in {:.2?}",
            solution_steps.len(),
            elapsed,
        );
    } else {
        println!("Incomplete solution!");
        println!(" This signals an issue with a solver, which thought that it found a solution but for some buggy reason it ends up not being one here.")
    }

    solved_system.print_system_state();
    Ok(())
}
