use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::game_elements::{
    step::Step,
    glass_system::{GlassSystem, GlassSystemError}
};
use crate::solver::solution::WaterSortSolution;

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
}

pub type SolverResult<T> = Result<T, SolverError>;

pub struct Solver;

#[derive(PartialEq, Eq, Hash)]
pub struct Neighbour {
    /// The resulting system
    pub system: GlassSystem,
    /// The step that leads to this resulting system
    pub step: Step,
}

impl Solver {
    /// BFS for constructing the valid-steps graph.
    pub fn find_solution(&self, start_system: &GlassSystem) -> SolverResult<WaterSortSolution> {
        let full_now = Instant::now();
        let mut paths: HashMap<GlassSystem, (Step, GlassSystem)> = HashMap::new();
        let mut queue: VecDeque<GlassSystem> = VecDeque::new();
        let mut found_systems: HashSet<GlassSystem> = HashSet::new();

        let root = start_system.clone();
        queue.push_back(root.clone());
        found_systems.insert(root);

        let mut number_of_iterations = 0.;
        let mut number_of_extracted_neighbours = 0.;
        let mut sum_of_new_neighbours = 0.;
        let mut max_neighbours = 0;

        let mut neighbour_building = Duration::new(0, 0);
        let mut neighbour_extraction = Duration::new(0, 0);

        while !queue.is_empty() {
            number_of_iterations += 1.;
            // Unwrap is fair due to the while condition
            let node = queue.pop_front().unwrap();

            let now = Instant::now();
            let mut neighbours = build_neighbours(&node);
            neighbour_building += now.elapsed();

            let now = Instant::now();
            // Collect necessary since iterators are lazy
            let extracted: Vec<_> = neighbours
                .extract_if(|n| found_systems.contains(&n.system))
                .collect();

            number_of_extracted_neighbours += extracted.len() as f64;
            neighbour_extraction += now.elapsed();

            sum_of_new_neighbours += neighbours.len() as f64;
            if neighbours.len() > max_neighbours {
                max_neighbours = neighbours.len()
            }

            for neighbour in neighbours {
                queue.push_back(neighbour.system.clone());
                found_systems.insert(neighbour.system.clone());

                paths.insert(neighbour.system.clone(), (neighbour.step.clone(), node.clone()));

                if neighbour.system.is_solved() {
                    println!("Avg. new neighbours per loop: {:.4} ({}/{})", sum_of_new_neighbours / number_of_iterations, sum_of_new_neighbours, number_of_iterations);
                    println!("Avg. extracted neighbours per loop: {:.4} ({}/{})", number_of_extracted_neighbours / number_of_iterations, number_of_extracted_neighbours, number_of_iterations);
                    println!("Most new neighbours: {}", max_neighbours);
                    println!("Time spent on building neighbours: {:.2?}", neighbour_building);
                    println!("Time spent on extracting neighbours: {:.2?}", neighbour_extraction);
                    let solution = get_solution_path(&paths, &neighbour.system);
                    let full_time = full_now.elapsed();
                    println!("Total time spent: {:.2?}", full_time);
                    println!("Non-measured time: {:.2?}", full_time - neighbour_extraction -neighbour_building);
                    return Ok(solution);
                }
            }
        }

        Err(SolverError::NoSolutionError)
    }

    /// Given a possible solution, iterate through the steps and
    /// attempt to solve the game by mutating the starting system.
    pub fn solve(
        &self,
        mut start_system: GlassSystem,
        solution: &WaterSortSolution,
    ) -> SolverResult<GlassSystem> {
        for (idx, step) in solution.steps().iter().enumerate() {
            start_system
                .try_pour(step.source, step.destination)
                .map_err(|_| SolverError::InvalidSolution(idx))?;
        }
        Ok(start_system)
    }
}


/// Collects all valid steps and creates all neighbours for each possible step.
fn build_neighbours(system: &GlassSystem) -> HashSet<Neighbour> {
    let mut neighbours = HashSet::new();
    let valid_steps = system.get_valid_steps();
    for step in &valid_steps {
        let mut new_system = system.clone();
        if new_system.try_pour(step.source, step.destination).is_ok() {
            neighbours.insert(Neighbour {step: step.clone(), system: new_system});
        }
    }

    neighbours
}

fn get_solution_path(
    paths: &HashMap<GlassSystem, (Step, GlassSystem)>,
    solution_node: &GlassSystem,
) -> WaterSortSolution {
    let mut steps = WaterSortSolution::default();

    let mut current_node = solution_node.clone();
    let mut parent_node = paths.get(&current_node);

    while parent_node.is_some() {
        // Valid unwrap due to while condition.
        let (step, parent) = parent_node.unwrap();
        steps.push_front(step.clone());
        current_node = parent.clone();
        parent_node = paths.get(&current_node);
    }

    steps
}