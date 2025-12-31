use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use thiserror::Error;

use crate::game_elements::glass_system::{GlassSystem, GlassSystemError};
use crate::solver::{node::Node, solution::WaterSortSolution};

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

impl Solver {
    /// BFS for constructing the valid-steps graph.
    pub fn find_solution(&self, start_system: &GlassSystem) -> SolverResult<WaterSortSolution> {
        let full_now = Instant::now();
        let mut paths: HashMap<Node, Node> = HashMap::new();
        let mut queue: VecDeque<Node> = VecDeque::new();
        let mut in_queue: HashSet<Node> = HashSet::new();
        // TODO: Next to avoid copying each system config should also have just an ID to avoid heavy clones.
        // let mut system_map: HashMap<GlassSystem, u64> = HashMap::new();

        let mut root = Node::from(start_system.clone());
        root.build_neighbours();
        queue.push_back(root.clone());
        in_queue.insert(root);

        let mut number_of_iterations = 0.;
        let mut number_of_extracted_neighbours = 0.;
        let mut sum_of_neighbours = 0.;
        let mut max_neighbours = 0;

        let mut neighbour_building = Duration::new(0, 0);
        let mut neighbour_extraction = Duration::new(0, 0);
        let mut valid_step_time = Duration::new(0, 0);
        let mut step_insert_time = Duration::new(0, 0);

        while !queue.is_empty() {
            number_of_iterations += 1.;
            // Unwrap is fair due to the while condition
            let mut node = queue.pop_front().unwrap();

            let now = Instant::now();
                // Collect necessary since iterators are lazy
                let extracted: Vec<_> = node
                .neighbour_nodes
                .extract_if(|_, v| in_queue.contains(v))
                .collect();
            number_of_extracted_neighbours += extracted.len() as f64;
            neighbour_extraction += now.elapsed();

            sum_of_neighbours += node.neighbour_nodes.keys().len() as f64;
            if node.neighbour_nodes.keys().len() > max_neighbours {
                max_neighbours = node.neighbour_nodes.keys().len()
            }
            // Double for loop necessary due to borrow checker.
            for next_neighbour in node.neighbour_nodes.values_mut() {
                let now = Instant::now();
                let (valid_time, insert_time) = next_neighbour.build_neighbours();
                valid_step_time += valid_time;
                step_insert_time += insert_time;
                neighbour_building += now.elapsed();
            }

            for next_neighbour in node.neighbour_nodes.values() {
                queue.push_back(next_neighbour.clone());
                in_queue.insert(next_neighbour.clone());
                paths.insert(next_neighbour.clone(), node.clone());
                if next_neighbour.system.is_solved() {
                    println!("Avg. new neighbours per loop: {:.4} ({}/{})", sum_of_neighbours / number_of_iterations, sum_of_neighbours, number_of_iterations);
                    println!("Avg. extracted neighbours per loop: {:.4} ({}/{})", number_of_extracted_neighbours / number_of_iterations, number_of_extracted_neighbours, number_of_iterations);
                    println!("Most new neighbours: {}", max_neighbours);
                    println!("Time spent on building neighbours: {:.2?}", neighbour_building);
                    println!("Time spent on extracting neighbours: {:.2?}", neighbour_extraction);
                    println!("Time spent on valid step neighbours: {:.2?}", valid_step_time);
                    println!("Time spent on insert neighbours: {:.2?}", step_insert_time);
                    let solution = self.get_solution_path(&paths, next_neighbour);
                    let full_time = full_now.elapsed();
                    println!("Total time spent: {:.2?}", full_time);
                    println!("Non-measured time: {:.2?}", full_time - step_insert_time - valid_step_time - neighbour_extraction -neighbour_building);
                    return Ok(solution);
                }
            }
        }

        Err(SolverError::NoSolutionError)
    }

    fn get_solution_path(
        &self,
        paths: &HashMap<Node, Node>,
        solution_node: &Node,
    ) -> WaterSortSolution {
        let mut steps = WaterSortSolution::default();

        let mut current_node = solution_node.clone();
        let mut parent = paths.get(&current_node);

        while parent.is_some() {
            // Valid unwrap due to while condition.
            let unwrapped_parent = parent.unwrap();
            // Unwrap is safe since the parent can only be parent if current is a neighbour
            // Note however that this relies on the fact that the two neighbour-management
            // fields are kept in sync.
            let step = unwrapped_parent.neighbour_steps.get(&current_node).unwrap();
            steps.push_front(step.clone());
            current_node = unwrapped_parent.clone();
            parent = paths.get(&current_node);
        }

        steps
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
