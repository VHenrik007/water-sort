use std::collections::{HashMap, HashSet, VecDeque};
use std::fmt::Display;
use std::hash::{Hash, Hasher};

use crate::solver::glass_system::{GlassSystem, GlassSystemResult};

pub struct Solver;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Step {
    source: usize,
    destination: usize,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.source, self.destination)
    }
}

#[derive(Clone)]
struct Node {
    // TODO: Make this a hash key instead
    system: GlassSystem,
    neighbour_nodes: HashMap<Step, Node>,
    neighbour_steps: HashMap<Node, Step>,
    
}

impl Hash for Node {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.system.hash(state);
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.system == other.system
    }
}

impl Eq for Node {}

impl From<GlassSystem> for Node {
    fn from(system: GlassSystem) -> Self {
        Node {
            system,
            neighbour_nodes: HashMap::new(),
            neighbour_steps: HashMap::new(),
        }
    }
}

impl Node {
    /// Creates a new node from a given system associated with possible steps by
    /// generating the possible new systems that can arise from them.
    pub fn new(system: GlassSystem, possible_steps: Vec<Step>) -> Self {
        let mut node = Node::from(system);
        let (neighbour_nodes, neighbour_steps) = node.build_neighbours(possible_steps);
        for (step, neighbour) in &neighbour_nodes {
            node.neighbour_nodes
                .insert(step.clone(), Node::from(neighbour.clone()));
        }
        for (neighbour, step) in &neighbour_steps {
            node.neighbour_steps
                .insert(Node::from(neighbour.clone()), step.clone());
        }
        node
    }

    /// Returns the resulting systems with the given possible steps.
    /// TODO: Could be its own function instead of class method
    ///       that takes only the start system even.
    pub fn build_neighbours(&self, possible_steps: Vec<Step>) -> (HashMap<Step, GlassSystem>, HashMap<GlassSystem, Step>) {
        let mut neighbour_nodes = HashMap::new();
        let mut neighbour_steps = HashMap::new();
        for step in &possible_steps {
            let mut new_system = self.system.clone();
            // Should be always ok, but sometimes (often) it isn't. TODO: Try else branch debug
            if new_system.try_pour(step.source, step.destination).is_ok() {
                neighbour_nodes.insert(step.clone(), new_system.clone());
                neighbour_steps.insert(new_system, step.clone());
            }
        }
        (neighbour_nodes, neighbour_steps)
    }
}

impl Solver {
    pub fn solve(&self, start_system: &GlassSystem) -> GlassSystemResult<VecDeque<Step>> {
        // Solution FOUND! Nice, TODO: let's make it usable and then refactor.
        let mut has_checked: HashSet<Node> = HashSet::new();
        let mut distances: HashMap<Node, u32> = HashMap::new();
        let mut paths: HashMap<Node, Node> = HashMap::new();
        let mut queue: VecDeque<Node> = VecDeque::new();

        let valid_steps = self.get_valid_steps(start_system);
        let root = Node::new(start_system.clone(), valid_steps);
        distances.insert(root.clone(), 0);

        for (step, neighbour) in &root.neighbour_nodes {
            distances.insert(neighbour.clone(), distances.get(&root).unwrap() + 1);
            paths.insert(neighbour.clone(), root.clone());
            if neighbour.system.is_solved() {
                println!("FOUND SOLUTION!");
                let mut result = VecDeque::new();
                result.push_front(step.clone());
                return Ok(result);
            }
            queue.push_back((*neighbour).clone());
        }
        has_checked.insert(root.clone());

        while !queue.is_empty() {
            // Unwrap is fair due to the while condition
            let mut node = queue.pop_front().unwrap();
            has_checked.insert(node.clone());

            let valid_steps = self.get_valid_steps(&node.system);
            let (next_neighbours, _) = node.build_neighbours(valid_steps);
            for (next_step, next_neighbour) in next_neighbours {
                let maybe_new_node = Node::from(next_neighbour);
                if has_checked.contains(&maybe_new_node) || queue.contains(&maybe_new_node) {
                    continue;
                }
                queue.push_back(maybe_new_node.clone());
                node.neighbour_nodes.insert(next_step.clone(), maybe_new_node.clone());
                node.neighbour_steps.insert(maybe_new_node.clone(), next_step);
                distances.insert(maybe_new_node.clone(), distances.get(&node).unwrap() + 1);
                paths.insert(maybe_new_node.clone(), node.clone());
                if maybe_new_node.system.is_solved() {
                    let solution = self.get_solution_path(&paths, maybe_new_node);
                    println!("FOUND SOLUTION OF LENGTH {}!", solution.len());
                    return Ok(solution);
                }
            }
        }
        println!("SOLUTION NOT FOUND");
        Ok(VecDeque::new())
    }
    
    fn get_solution_path(
        &self,
        paths: &HashMap<Node, Node>,
        solution_node: Node
    ) -> VecDeque<Step> {
        let mut steps = VecDeque::new();
        let mut current_node = solution_node;
        let mut parent = paths.get(&current_node);
        
        while parent.is_some() {
            let unwrapped_parent = parent.unwrap();
            let step = unwrapped_parent.neighbour_steps.get(&current_node).unwrap();
            steps.push_front(step.clone());
            current_node = unwrapped_parent.clone();
            parent = paths.get(&current_node);
        }
        
        steps
    }

    /// Gets all valid steps for a given node (or system)
    fn get_valid_steps(&self, system: &GlassSystem) -> Vec<Step> {
        let mut valid_steps = Vec::new();

        for source in system.get_state() {
            if source.is_empty() {
                continue;
            }
            for destination in system.get_state() {
                if destination.is_full() {
                    continue;
                }

                if destination.is_empty() || source.top() == destination.top() {
                    valid_steps.push(Step {
                        source: source.id,
                        destination: destination.id,
                    })
                }
            }
        }

        valid_steps
    }
    
    pub fn validate_solution(&self, mut start_system: GlassSystem, steps: &VecDeque<Step>) -> GlassSystemResult<GlassSystem> {
        for step in steps {
            start_system.try_pour(step.source, step.destination)?;
        }
        Ok(start_system)
    }
}
