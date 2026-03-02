use std::time::Instant;
use thiserror::Error;

use water_sort::{
    cli_arguments::{Args, ArgumentsError, ProgramGoalArg, SearchMethod}, game_elements::glass_system::GlassSystem, solver::{SolverError, bfs_shortest_path, build_state_graph, heuristic_dijkstra_search, solve}
};

/// Custom error for the solver.
#[derive(Debug, Error)]
pub enum WaterSortError {
    /// Derived from the ArgumentsError if an issue happens on that level.
    #[error(transparent)]
    ArgumentsError(#[from] ArgumentsError),
    /// Derived from the SolverError if an issue happens on that level.
    #[error(transparent)]
    SolverError(#[from] SolverError),
}

pub type WaterSortResult<T> = Result<T, WaterSortError>;

fn main() -> WaterSortResult<()> {
    let (args, system_to_solve) = Args::process_arguments()?;
    system_to_solve.print_system_state();

    match args.program_goal {
        ProgramGoalArg::ExploreSystem => {
            let graph = build_state_graph(&system_to_solve, &args.heuristic_evaluation.clone().into(), args.max_depth);
            if let Some(path) = args.output_path {
                graph.write_to_file(path)?;
            }
        },
        ProgramGoalArg::SolveSystem => {
            solve_system(args, system_to_solve)?
        }
    }

    Ok(())
}

fn solve_system(args: Args, system_to_solve: GlassSystem) -> WaterSortResult<()> {
    println!("Solving...");

    let now = Instant::now();
    let solution_steps = match args.search_method {
        SearchMethod::Bfs => bfs_shortest_path(&system_to_solve),
        SearchMethod::Heuristic => {
            heuristic_dijkstra_search(&system_to_solve, &args.heuristic_evaluation.into())
        }
    }?;
    let elapsed = now.elapsed();

    println!("Steps:");
    println!("{}", solution_steps);

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
