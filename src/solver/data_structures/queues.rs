use std::collections::{BinaryHeap, VecDeque};

/// Different type of queues for graph-traversal algorithms.
/// This is used as a convenience, essentially a renaming wrapper
/// so that all operations have the same name.
#[allow(dead_code)]
pub enum QueueType<T> {
    /// Regular queue
    Regular(VecDeque<T>),
    /// Stack (reversed queue)
    Stack(VecDeque<T>),
    /// Priority queue allowing heuristics to enter the picture.
    /// Note that the underlying binary heap is maximum by default,
    /// and custom `cmp` had to be written to make it a minimum heap.
    Priority(BinaryHeap<T>),
}

/// Wrapper type over the enum to allow the commonly named methods.
pub struct SolverQueue<T> {
    /// The queue enum variant.
    queue: QueueType<T>,
}

impl<T: Ord> SolverQueue<T> {
    /// Constructs the type for the regular queue.
    pub fn regular() -> Self {
        SolverQueue {
            queue: QueueType::Regular(VecDeque::new()),
        }
    }

    /// Constructs the type for the priority queue.
    pub fn priority() -> Self {
        SolverQueue {
            queue: QueueType::Priority(BinaryHeap::new()),
        }
    }

    /// General push method. For priority queue it's trivial, and in
    /// case of regular queues we push to the back and pop from the front.
    pub fn push(&mut self, elem: T) {
        match self.queue {
            QueueType::Priority(ref mut queue) => {
                queue.push(elem);
            }
            QueueType::Stack(ref mut queue) => {
                queue.push_front(elem);
            }
            QueueType::Regular(ref mut queue) => {
                queue.push_back(elem);
            }
        }
    }

    /// General pop method. For priority queue it's trivial, and in
    /// case of regular queues we push to the back and pop from the front.
    pub fn pop(&mut self) -> Option<T> {
        match self.queue {
            QueueType::Regular(ref mut queue) => queue.pop_front(),
            QueueType::Stack(ref mut queue) => queue.pop_front(),
            QueueType::Priority(ref mut queue) => queue.pop(),
        }
    }

    /// Len is the same, but it is more convenient to have it this way.
    pub fn len(&self) -> usize {
        match &self.queue {
            QueueType::Regular(queue) => queue.len(),
            QueueType::Stack(queue) => queue.len(),
            QueueType::Priority(queue) => queue.len(),
        }
    }

    /// Emptiness is the same, but it is more convenient to have it this way.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
