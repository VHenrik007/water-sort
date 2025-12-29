use std::{
    hash::{Hash, Hasher},
    collections::HashMap,
};

use crate::game_elements::{
    glass_system::GlassSystem,
    step::Step,
};


#[derive(Clone)]
pub struct Node {
    pub system: GlassSystem,
    pub neighbour_nodes: HashMap<Step, Node>,
    pub neighbour_steps: HashMap<Node, Step>,
    
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
    /// Collects all valid steps and creates all neighbours for each possible step.
    pub fn build_neighbours(&mut self) {
        for step in &self.system.get_valid_steps() {
            let mut new_system = self.system.clone();
            // Should be always ok, but sometimes (often) it isn't. TODO: Try else branch debug
            if new_system.try_pour(step.source, step.destination).is_ok() {
                self.neighbour_nodes.insert(step.clone(), Node::from(new_system.clone()));
                self.neighbour_steps.insert(Node::from(new_system), step.clone());
            }
        }
    }
}