use std::collections::VecDeque;

use crate::game_elements::step::Step;

#[derive(Default)]
pub struct WaterSortSolution(VecDeque<Step>);

impl WaterSortSolution {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn push_front(&mut self, step: Step) {
        self.0.push_front(step);
    }

    pub fn steps(&self) -> &VecDeque<Step> {
        &self.0
    }
}
