use thiserror::Error;

use water_sort::game_elements::{color::Color, glass::Glass};
use water_sort::{
    game_elements::{
        glass::GlassError,
        glass_system::{GlassSystem, GlassSystemError},
    },
    solver::solver::{Solver, SolverError},
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
}

pub type WaterSortResult<T> = Result<T, WaterSortError>;

fn main() -> WaterSortResult<()> {
    let glass1 = Glass::new();
    let glass2 = Glass::new();
    let glass3 = Glass::from_colors(&[Color::BLUE, Color::BLUE, Color::GREEN, Color::GREEN])?;
    let glass4 = Glass::from_colors(&[Color::BLUE, Color::RED, Color::GREEN, Color::GREEN])?;
    let glass5 = Glass::from_colors(&[Color::BLUE, Color::RED, Color::YELLOW, Color::YELLOW])?;
    let glass6 = Glass::from_colors(&[Color::YELLOW, Color::YELLOW, Color::RED, Color::RED])?;

    let pre_system = &[glass1, glass2, glass3, glass4, glass5, glass6];

    let system = GlassSystem::new(pre_system.to_vec());
    system.print_system_state();

    println!("Solving...");
    let solver = Solver {};
    let solution_steps = solver.find_solution(&system)?;
    let solved_system = solver.solve(system, &solution_steps)?;
    if solved_system.is_solved() {
        println!("Solved in {} steps", solution_steps.len());
    } else {
        println!("Incomplete solution!")
    }
    solved_system.print_system_state();

    Ok(())
}
