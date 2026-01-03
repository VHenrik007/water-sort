use crate::solver::{evaluation::SolutionValue, SystemId};

/// These should be stored in any solver's main queue.
pub struct EvaluatedNode {
    /// ID of the unique glass system.
    pub system_id: SystemId,
    /// Some metric determining how good this state is compared to a final solution.
    /// The exact metric is defined by a separate module responsible for evaluation.
    pub solution_value: SolutionValue,
}

impl PartialEq for EvaluatedNode {
    fn eq(&self, other: &Self) -> bool {
        self.solution_value.eq(&other.solution_value)
    }
}

impl Eq for EvaluatedNode {}

impl Ord for EvaluatedNode {
    /// To make the binary heap a minimum heap we switch order of comparison here
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .solution_value
            .cmp(&self.solution_value)
            .then_with(|| self.system_id.cmp(&other.system_id))
    }
}

impl PartialOrd for EvaluatedNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
