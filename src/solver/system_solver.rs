use std::collections::{HashMap, HashSet, VecDeque};
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
        let mut has_checked: HashSet<Node> = HashSet::new();
        let mut paths: HashMap<Node, Node> = HashMap::new();
        let mut queue: VecDeque<Node> = VecDeque::new();

        let mut root = Node::from(start_system.clone());
        root.build_neighbours();
        queue.push_back(root);
        // TODO: More stats on neighoburs!!!!!!!!!!
        let mut number_of_iterations = 0.;
        let mut number_of_extracted_neighbours = 0.;
        let mut sum_of_neighbours = 0.;
        while !queue.is_empty() {
            number_of_iterations += 1.;
            // Unwrap is fair due to the while condition
            let mut node = queue.pop_front().unwrap();
            has_checked.insert(node.clone());
            // Collect necessary since iterators are lazy
            let extracted: Vec<_> = node
                .neighbour_nodes
                .extract_if(|_, v| has_checked.contains(v) || queue.contains(v))
                .collect();
            number_of_extracted_neighbours += extracted.len() as f64;

            sum_of_neighbours += node.neighbour_nodes.keys().len() as f64;
            // Double for loop necessary due to borrow checker.
            for next_neighbour in node.neighbour_nodes.values_mut() {
                next_neighbour.build_neighbours();
            }

            for next_neighbour in node.neighbour_nodes.values() {
                queue.push_back(next_neighbour.clone());
                paths.insert(next_neighbour.clone(), node.clone());
                if next_neighbour.system.is_solved() {
                    println!("Avg. neighbours per loop: {:.4} ({}/{})", sum_of_neighbours / number_of_iterations, sum_of_neighbours, number_of_iterations);
                    println!("Avg. extracted neighbours per loop: {:.4} ({}/{})", number_of_extracted_neighbours / number_of_iterations, number_of_extracted_neighbours, number_of_iterations);
                    return Ok(self.get_solution_path(&paths, next_neighbour));
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
