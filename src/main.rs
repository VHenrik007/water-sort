use std::time::Instant;
use thiserror::Error;

use water_sort::{
    game_elements::{glass::GlassError, glass_system::GlassSystemError},
    generate::system_generator::{generate_random_system_with_seed, SystemGeneratorError},
    solver::system_solver::{Solver, SolverError},
};

/// Custom error for the solver.
#[derive(Debug, Error)]
pub enum WaterSortError {
    /// Derived from the GlassError if an issue happens on that level.
    #[error(transparent)]
    GlassError(#[from] GlassError),
    /// Derived from the GlassSystemError if an issue happens on that level.
    #[error(transparent)]
    GlassSystemError(#[from] GlassSystemError),
    /// Derived from the SolverError if an issue happens on that level.
    #[error(transparent)]
    SolverError(#[from] SolverError),
    /// Derived from the SystemGeneratorError if an issue happens on that level.
    #[error(transparent)]
    SystemGeneratorError(#[from] SystemGeneratorError),
}

pub type WaterSortResult<T> = Result<T, WaterSortError>;

fn main() -> WaterSortResult<()> {
    let system = generate_random_system_with_seed(4, 43)?;
    system.print_system_state();

    println!("Solving...");
    let solver = Solver {};

    let now = Instant::now();
    let solution_steps = solver.find_solution(&system)?;
    let elapsed = now.elapsed();

    let solved_system = solver.solve(system, &solution_steps)?;
    if solved_system.is_solved() {
        println!(
            "Found a {} step solution in {:.2?}",
            solution_steps.len(),
            elapsed
        );
    } else {
        println!("Incomplete solution!")
    }

    solved_system.print_system_state();
    Ok(())
}
