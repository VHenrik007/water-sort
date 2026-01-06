use clap::Parser;
use std::time::Instant;
use thiserror::Error;

use water_sort::{
    game_elements::{glass::GlassError, glass_system::GlassSystemError},
    generate::system_generator::{generate_random_system_with_seed, SystemGeneratorError},
    solver::{bfs_shortest_path, heuristic_dijkstra_search, solve, SolutionValueMode, SolverError},
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

#[derive(Debug, Clone)]
enum SearchMethod {
    Bfs,
    Heuristic,
}

impl clap::ValueEnum for SearchMethod {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Bfs, Self::Heuristic]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Bfs => Some(clap::builder::PossibleValue::new("BFS")),
            Self::Heuristic => Some(clap::builder::PossibleValue::new("Heuristic")),
        }
    }
}

#[derive(Debug, Clone)]
enum HeuristicEvaluation {
    Constant,
    ColorCounting,
    ColorAlternation,
}

impl clap::ValueEnum for HeuristicEvaluation {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Constant, Self::ColorCounting, Self::ColorAlternation]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Constant => Some(clap::builder::PossibleValue::new("Constant")),
            Self::ColorCounting => Some(clap::builder::PossibleValue::new("ColorCounting")),
            Self::ColorAlternation => Some(clap::builder::PossibleValue::new("ColorAlternation")),
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// The search method: ["BFS", "Heuristic"]
    #[arg(short, long)]
    search_method: SearchMethod,

    /// The search method: ["constant (dfs)", "color-count", "alternatin-colors"]
    #[arg(short, long)]
    heuristic_evaluation: HeuristicEvaluation,

    /// Random seed for the system genration
    #[arg(short, long)]
    random_seed: u64,

    /// Number of colors
    #[arg(short, long)]
    number_of_colors: usize,
}

fn main() -> WaterSortResult<()> {
    let args = Args::parse();

    let system = generate_random_system_with_seed(args.number_of_colors, args.random_seed)?;
    system.print_system_state();

    println!("Solving...");

    let now = Instant::now();
    let solution_steps = match args.search_method {
        SearchMethod::Bfs => bfs_shortest_path(&system),
        SearchMethod::Heuristic => {
            let heuristic = match args.heuristic_evaluation {
                HeuristicEvaluation::Constant => SolutionValueMode::Constant,
                HeuristicEvaluation::ColorCounting => SolutionValueMode::ColorCount,
                HeuristicEvaluation::ColorAlternation => SolutionValueMode::AlternatingColors,
            };
            heuristic_dijkstra_search(&system, &heuristic)
        }
    }?;
    let elapsed = now.elapsed();

    let solved_system = solve(system, &solution_steps)?;

    if solved_system.is_solved() {
        println!(
            "Found a {} step solution in {:.2?}",
            solution_steps.len(),
            elapsed,
        );
    } else {
        println!("Incomplete solution!")
    }

    solved_system.print_system_state();
    Ok(())
}
