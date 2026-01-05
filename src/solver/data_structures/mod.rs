use std::collections::HashMap;

use crate::{game_elements::step::Step, solver::SystemId};

mod node;
mod queues;
mod system_dictionary;

pub use node::EvaluatedNode;
pub use queues::SolverQueue;
pub use system_dictionary::SystemDictionary;

/// A neighbour is a struct stored for determining paths. It is essentially
/// a tuple of a system and a step that lead there. Neighbours can only be
/// understood in context with a parent defined when constructing the graph.
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct Neighbour {
    /// The resulting system.
    pub system: SystemId,
    /// The step that leads to this resulting system.
    pub step: Step,
}

/// Alias type for clarity and future flexibility.
pub type Paths = HashMap<SystemId, (Step, SystemId)>;
