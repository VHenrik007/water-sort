mod data_structures;
mod evaluation;
mod solution;
mod system_solver;
mod graph_builder;

use std::io::Error as IOError;

use thiserror::Error;

use crate::game_elements::glass_system::GlassSystemError;

pub use evaluation::{solution_value, SolutionValueMode};
pub use system_solver::{bfs_shortest_path, heuristic_dijkstra_search, solve};
pub use graph_builder::build_state_graph;

/// Custom error for the solver.
#[derive(Debug, Error)]
pub enum SolverError {
    /// Derived from the GlassSystemError if an issue happens on that level.
    #[error(transparent)]
    GlassSystemError(#[from] GlassSystemError),
    /// The solver might not find a valid solution in all cases.
    #[error("No solution found!")]
    NoSolutionError,
    /// Even if a solution is found, for some reason if that ends up
    /// invalid, this error might be useful for debugging.
    #[error("Invalid solution at step {0}")]
    InvalidSolution(usize),
    /// File handling error.
    #[error(transparent)]
    IOError(#[from] IOError)
}

pub type SolverResult<T> = Result<T, SolverError>;
pub type SystemId = u32;
