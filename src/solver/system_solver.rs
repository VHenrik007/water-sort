use std::collections::HashSet;

use crate::game_elements::glass_system::GlassSystem;
use crate::solver::data_structures::{
    EvaluatedNode, Neighbour, Paths, SolverQueue, SystemDictionary,
};
use crate::solver::solution::WaterSortSolution;
use crate::solver::{solution_value, SolutionValueMode, SolverResult, SystemId};

/// Regular BFS, guaranteed shortest path, but takes ages.
pub fn bfs_shortest_path(start_system: &GlassSystem) -> SolverResult<WaterSortSolution> {
    let mut queue: SolverQueue<SystemId> = SolverQueue::regular();
    let mut paths: Paths = Paths::new();

    // Maps between systems and their IDs. Used to avoid too much cloning of entire states, as well as
    // checking if a given system is already found or not (although paths could be used for that too.)
    // TODO: A different graph-building approach would be to always only store the steps
    //       and at each evaluation reconstruct the system. To me this sounds intuitively
    //       slower but memory-wise it would definitely be much better.
    let mut system_dictionary = SystemDictionary::new();
    system_dictionary.add_system(start_system.clone());

    queue.push(*system_dictionary.get_id(start_system).unwrap());

    while !queue.is_empty() {
        let node = queue.pop().unwrap();

        let neighbours = build_neighbours(node, &mut system_dictionary);

        for neighbour in neighbours {
            queue.push(neighbour.system);
            paths.insert(neighbour.system, (neighbour.step.clone(), node));

            if system_dictionary
                .get_system(&neighbour.system)
                .unwrap()
                .is_solved()
            {
                return Ok(retrace_solution_path(&paths, &neighbour.system));
            }
        }
    }

    Err(super::SolverError::NoSolutionError)
}

/// Regular DFS, can be very fast but might find a very long path.
pub fn dfs_shortest_path(start_system: &GlassSystem) -> SolverResult<WaterSortSolution> {
    let mut queue: SolverQueue<SystemId> = SolverQueue::stack();
    let mut paths: Paths = Paths::new();

    let mut system_dictionary = SystemDictionary::new();
    system_dictionary.add_system(start_system.clone());

    queue.push(*system_dictionary.get_id(start_system).unwrap());

    while !queue.is_empty() {
        let node = queue.pop().unwrap();

        let neighbours = build_neighbours(node, &mut system_dictionary);

        for neighbour in neighbours {
            queue.push(neighbour.system);
            paths.insert(neighbour.system, (neighbour.step.clone(), node));

            if system_dictionary
                .get_system(&neighbour.system)
                .unwrap()
                .is_solved()
            {
                return Ok(retrace_solution_path(&paths, &neighbour.system));
            }
        }
    }

    Err(super::SolverError::NoSolutionError)
}

/// Dijkstra-like for constructing the valid-steps graph using heuristics to narrow the search.
/// This does not guarantee shortest path.
/// NOTE: Using constant evaluation mode is more or less equivalent with the DFS approach.
//  TODO: Look up how A* is different.
pub fn heuristic_dijkstra_search(
    start_system: &GlassSystem,
    evaluation_mode: &SolutionValueMode,
) -> SolverResult<WaterSortSolution> {
    let mut queue: SolverQueue<EvaluatedNode> = SolverQueue::priority();
    let mut paths: Paths = Paths::new();

    // Maps between systems and their IDs. Used to avoid too much cloning of entire states, as well as
    // checking if a given system is already found or not (although paths could be used for that too.)
    // TODO: A different graph-building approach would be to always only store the steps
    //       and at each evaluation reconstruct the system. To me this sounds intuitively
    //       slower but memory-wise it would definitely be much better.
    let mut system_dictionary = SystemDictionary::new();
    system_dictionary.add_system(start_system.clone());

    queue.push(EvaluatedNode {
        system_id: *system_dictionary.get_id(start_system).unwrap(),
        solution_value: solution_value(start_system, evaluation_mode),
    });

    while !queue.is_empty() {
        let node = queue.pop().unwrap();

        let neighbours = build_neighbours(node.system_id, &mut system_dictionary);

        for neighbour in neighbours {
            let neighbour_node = EvaluatedNode {
                system_id: neighbour.system,
                solution_value: solution_value(
                    system_dictionary.get_system(&neighbour.system).unwrap(),
                    evaluation_mode,
                ),
            };
            queue.push(neighbour_node);

            paths.insert(neighbour.system, (neighbour.step.clone(), node.system_id));

            if system_dictionary
                .get_system(&neighbour.system)
                .unwrap()
                .is_solved()
            {
                return Ok(retrace_solution_path(&paths, &neighbour.system));
            }
        }
    }

    Err(super::SolverError::NoSolutionError)
}

/// Given a possible solution, iterate through the steps and
/// attempt to solve the game by mutating the starting system.
pub fn solve(
    mut start_system: GlassSystem,
    solution: &WaterSortSolution,
) -> SolverResult<GlassSystem> {
    for (idx, step) in solution.steps().iter().enumerate() {
        start_system
            .try_pour(step.source, step.destination)
            .map_err(|_| super::SolverError::InvalidSolution(idx))?;
    }
    Ok(start_system)
}

/// Collects all valid steps and creates all new undiscovered neighbours for each possible step.
fn build_neighbours(
    system_id: SystemId,
    system_dictionary: &mut SystemDictionary,
) -> HashSet<Neighbour> {
    let mut neighbours = HashSet::new();

    let system = system_dictionary.get_system(&system_id).unwrap().clone();
    let valid_steps = system.get_valid_steps();

    for step in &valid_steps {
        let mut next_system = system.clone();

        if next_system.try_pour(step.source, step.destination).is_err() {
            println!("Some pouring failed, this should not happen!");
            println!(
                "If solutions are not found then this is a blocking issue, otherwise just warning."
            );
            continue;
        }

        if system_dictionary.get_id(&next_system).is_none() {
            neighbours.insert(Neighbour {
                step: step.clone(),
                system: system_dictionary.add_system(next_system),
            });
        }
    }

    neighbours
}

/// Builds the solution based on the graph path.
fn retrace_solution_path(paths: &Paths, solution_node: &SystemId) -> WaterSortSolution {
    let mut steps = WaterSortSolution::new();

    let mut parent_node = paths.get(solution_node);
    while let Some((step, parent)) = parent_node {
        steps.push(step.clone());
        parent_node = paths.get(parent);
    }

    steps
}
