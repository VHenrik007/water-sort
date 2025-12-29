use std::collections::{HashMap, HashSet, VecDeque};

use crate::game_elements::{
    glass_system::{GlassSystem, GlassSystemResult},
    step::Step,
};
use crate::solver::{
    node::Node,
};

pub struct Solver;

impl Solver {
    pub fn solve(&self, start_system: &GlassSystem) -> GlassSystemResult<VecDeque<Step>> {
        let mut has_checked: HashSet<Node> = HashSet::new();
        let mut paths: HashMap<Node, Node> = HashMap::new();
        let mut queue: VecDeque<Node> = VecDeque::new();

        let mut root = Node::from(start_system.clone());
        root.build_neighbours();
        queue.push_back(root);

        while !queue.is_empty() {
            // Unwrap is fair due to the while condition
            let mut node = queue.pop_front().unwrap();
            has_checked.insert(node.clone());
            let _: Vec<_> = node.neighbour_nodes.extract_if(|_, v| has_checked.contains(v) || queue.contains(v)).collect();
        
            // Double for loop necessary due to borrow checker.
            for (_, next_neighbour) in &mut node.neighbour_nodes {
                if has_checked.contains(&next_neighbour) || queue.contains(&next_neighbour) {
                    continue;
                }
        
                next_neighbour.build_neighbours();
            }

            for (_, next_neighbour) in &node.neighbour_nodes {
                if has_checked.contains(&next_neighbour) || queue.contains(&next_neighbour) {
                    continue;
                }

                queue.push_back(next_neighbour.clone());
                paths.insert(next_neighbour.clone(), node.clone());
                if next_neighbour.system.is_solved() {
                    let solution = self.get_solution_path(&paths, &next_neighbour);
                    return Ok(solution);
                }
            }
        }
        Ok(VecDeque::new())
    }
    
    fn get_solution_path(
        &self,
        paths: &HashMap<Node, Node>,
        solution_node: &Node
    ) -> VecDeque<Step> {
        let mut steps = VecDeque::new();
        let mut current_node = solution_node.clone();
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
    
    pub fn validate_solution(&self, mut start_system: GlassSystem, steps: &VecDeque<Step>) -> GlassSystemResult<GlassSystem> {
        for step in steps {
            start_system.try_pour(step.source, step.destination)?;
        }
        Ok(start_system)
    }
}
