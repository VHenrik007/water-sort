use std::{cmp, collections::{HashSet, VecDeque}, fmt::Display, fs, hash::Hash};

use crate::{game_elements::glass_system::GlassSystem, solver::{SolutionValueMode, SolverResult, SystemId, data_structures::{Neighbour, SystemDictionary}, evaluation::SolutionValue, solution_value}};

#[derive(Eq, Default, Debug, Clone)]
pub struct GeneralNode {
    system: SystemId,
    solution_value: SolutionValue,
    is_solved: bool,
    neighbours: HashSet<Neighbour>,
}

impl Hash for GeneralNode {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.system.hash(state);
    }
}

impl PartialEq for GeneralNode {
    fn eq(&self, other: &Self) -> bool {
        self.system.eq(&other.system)
    }
}

impl PartialOrd for GeneralNode {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        self.system.partial_cmp(&other.system)
    }
}

impl Ord for GeneralNode {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.system.cmp(&other.system)
    }
}

enum QueueElement {
    NewLevel,
    System(SystemId)
}

fn graph_string<T: IntoIterator<Item = GeneralNode>>(nodes: T) -> String {
    let mut graph_repr = String::new();
    let connecting_str = " -> ";

    for node in nodes {
        if node.is_solved {
            graph_repr.push_str("[SOLUTION]");
        } else {
            graph_repr.push_str("[        ]");
        }
        graph_repr.push_str(&node.system.to_string());
        graph_repr.push('(');
        graph_repr.push_str(&node.solution_value.to_string());
        graph_repr.push(')');
        graph_repr.push_str(": ");

        for neighbour in &node.neighbours {
            graph_repr.push('[');
            graph_repr.push_str(&neighbour.step.to_string());
            graph_repr.push_str("; ");
            graph_repr.push_str(&neighbour.system.to_string());
            graph_repr.push(']');
            graph_repr.push_str(&connecting_str);
        }

        if !&node.neighbours.is_empty() {
            for _ in 0..connecting_str.len() {
                graph_repr.pop();
            }
        } else {
            graph_repr.push_str("Dead end");
        }

        graph_repr.push('\n');
    }

    graph_repr
}

pub struct StateGraph {
    pub state_graph: HashSet<GeneralNode>,
    pub is_complete: bool,
}

impl StateGraph {
    fn new() -> Self {
        StateGraph { state_graph: HashSet::new(), is_complete: false }
    }

    fn insert(&mut self, node: GeneralNode) {
        self.state_graph.insert(node);
    }

    fn get_sorted(&self) -> Vec<GeneralNode> {
        let mut vec_form: Vec<GeneralNode> = self.state_graph.iter().cloned().collect();
        vec_form.sort_unstable();
        vec_form
    }

    pub fn write_to_file(&self, path: String) -> SolverResult<()> {
        let mut graph_str = graph_string(self.get_sorted());

        // TODO: Extend with longer summary and put at the beginning to make
        //       parsing easier.
        if self.is_complete {
            graph_str.push_str("Graph is complete!");
        } else {
            graph_str.push_str("Graph exploration ended at max depth");
        }

        if let Err(err) = fs::write(path, graph_str) {
            return Err(super::SolverError::IOError(err));
        }
        Ok(())
    }
}

impl Display for StateGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", graph_string(self.state_graph.iter().cloned()))
    }
}

pub fn build_state_graph(start_system: &GlassSystem, evaluation_mode: &SolutionValueMode, max_depth: u8) -> StateGraph {
    let mut graph = StateGraph::new();
    let mut queue: VecDeque<QueueElement> = VecDeque::new();
    let mut depth = 0;

    let mut system_dictionary = SystemDictionary::new();
    system_dictionary.add_system(start_system.clone());

    queue.push_back(
        QueueElement::System(*system_dictionary.get_id(start_system).unwrap()
    ));
    queue.push_back(QueueElement::NewLevel);

    while !queue.is_empty() && depth < max_depth {
        let node = match queue.pop_front().unwrap() {
            QueueElement::NewLevel => {
                depth += 1;
                queue.push_back(QueueElement::NewLevel);
                continue;
            }
            QueueElement::System(system) => system
        };

        let system = &system_dictionary.get_system(&node).unwrap();
        let mut graph_node = GeneralNode {
            system: node,
            solution_value: solution_value(system, evaluation_mode),
            is_solved: system.is_solved(),
            neighbours: HashSet::new()
        };

        let neighbours = build_neighbours(node, &mut system_dictionary);

        for neighbour in neighbours {
            queue.push_back(
                QueueElement::System(neighbour.system)
            );
            graph_node.neighbours.insert(neighbour);
        }

        graph.insert(graph_node);
    }
    graph.is_complete = depth < max_depth;
    graph
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
            panic!("Some pouring failed, this should not happen!");
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
