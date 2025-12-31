use std::collections::{HashMap, HashSet, VecDeque};
use std::hash::Hash;
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
type SystemId = u32;

pub struct Solver;

#[derive(PartialEq, Eq, Hash)]
pub struct Neighbour {
    /// The resulting system
    pub system: SystemId,
    /// The step that leads to this resulting system
    pub step: Step,
}

impl Solver {
    /// BFS for constructing the valid-steps graph.
    pub fn find_solution(&self, start_system: &GlassSystem) -> SolverResult<WaterSortSolution> {
        let full_now = Instant::now();
        let mut paths: HashMap<SystemId, (Step, SystemId)> = HashMap::new();
        let mut queue: VecDeque<SystemId> = VecDeque::new();
        let mut found_systems: HashSet<SystemId> = HashSet::new();

        let mut system_id_counter: SystemId = 0;
        let mut system_id_map: HashMap<GlassSystem, SystemId> = HashMap::new();
        let mut id_system_map: HashMap<SystemId, GlassSystem> = HashMap::new();

        let root = start_system.clone();

        system_id_map.insert(root.clone(), system_id_counter);
        id_system_map.insert(system_id_counter, root.clone());
        system_id_counter += 1;

        queue.push_back(*system_id_map.get(&root).unwrap());

        let mut number_of_iterations = 0.;
        let mut number_of_valid_steps = 0.;
        let mut sum_of_new_neighbours = 0.;
        let mut max_neighbours = 0;

        let mut neighbour_building = Duration::new(0, 0);

        while !queue.is_empty() {
            number_of_iterations += 1.;
            // Unwrap is fair due to the while condition
            let node = queue.pop_front().unwrap();

            let now = Instant::now();
            let (valid_steps, neighbours) = build_neighbours(node, &mut system_id_map, &mut id_system_map, &mut system_id_counter);
            neighbour_building += now.elapsed();

            number_of_valid_steps += valid_steps as f32;
            sum_of_new_neighbours += neighbours.len() as f32;
            if neighbours.len() > max_neighbours {
                max_neighbours = neighbours.len()
            }

            for neighbour in neighbours {
                queue.push_back(neighbour.system);
                found_systems.insert(neighbour.system);

                paths.insert(neighbour.system, (neighbour.step.clone(), node));

                if id_system_map.get(&neighbour.system).unwrap().is_solved() {
                    println!("Avg. new neighbours per loop: {:.4} ({}/{})", sum_of_new_neighbours / number_of_iterations, sum_of_new_neighbours, number_of_iterations);
                    println!("Avg. valid steps per loop: {:.4} ({}/{})", number_of_valid_steps / number_of_iterations, number_of_valid_steps, number_of_iterations);
                    println!("Most new neighbours: {}", max_neighbours);
                    println!("Time spent on building neighbours: {:.2?}", neighbour_building);
                    let solution = get_solution_path(&paths, &neighbour.system);
                    let full_time = full_now.elapsed();
                    println!("Total time spent: {:.2?}", full_time);
                    println!("Non-measured time: {:.2?}", full_time  - neighbour_building);
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
fn build_neighbours(system_id: SystemId, system_id_map: &mut HashMap<GlassSystem, SystemId>, id_system_map: &mut HashMap<SystemId, GlassSystem>, id_counter: &mut SystemId) -> (usize, HashSet<Neighbour>) {
    let mut neighbours = HashSet::new();
    let system = id_system_map.get(&system_id).unwrap().clone();
    let valid_steps = system.get_valid_steps();
    for step in &valid_steps {
        let mut new_system = system.clone();
        if new_system.try_pour(step.source, step.destination).is_ok() {
            let system_id = system_id_map.get(&new_system);
            if system_id.is_none() {
                system_id_map.insert(new_system.clone(), *id_counter);
                id_system_map.insert(*id_counter, new_system);
                neighbours.insert(Neighbour {step: step.clone(), system: *id_counter});
                *id_counter += 1;
            }
        }
    }

    (valid_steps.len(), neighbours)
}

fn get_solution_path(
    paths: &HashMap<SystemId, (Step, SystemId)>,
    solution_node: &SystemId,
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
