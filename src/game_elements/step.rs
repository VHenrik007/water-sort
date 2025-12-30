use std::fmt::Display;

#[derive(Hash, PartialEq, Eq, Clone)]
pub struct Step {
    pub source: usize,
    pub destination: usize,
}

impl Display for Step {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.source, self.destination)
    }
}
