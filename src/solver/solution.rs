use std::collections::VecDeque;

use crate::game_elements::step::Step;

/// A stack of steps. The decision with Stack is for simplicity as
/// the graph path tracing happens from the goal toward the start.
// TODO: Should contain the start system?
#[derive(Default)]
pub struct WaterSortSolution(VecDeque<Step>);

/// Each implementation is just to avoid using the `.0`.
impl WaterSortSolution {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push(&mut self, step: Step) {
        self.0.push_front(step);
    }

    pub fn steps(&self) -> &VecDeque<Step> {
        &self.0
    }

    pub fn new() -> Self {
        WaterSortSolution(VecDeque::new())
    }
}
