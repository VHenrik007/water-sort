use std::hash::{Hash, Hasher};

use thiserror::Error;

use crate::game_elements::{
    color::Color,
    glass::{Glass, GlassError},
    step::Step,
};

/// Custom error for systems.
#[derive(Debug, Error)]
pub enum GlassSystemError {
    /// For over/under indexing.
    #[error("Invalid glass indices: from:{0}, to:{1}, system length: {2}")]
    InvalidGlassIndices(usize, usize, usize),
    /// Derived from the GlassError if an issue happens on that level.
    #[error(transparent)]
    GlassError(#[from] GlassError),
}

/// Convenience.
pub type GlassSystemResult<T> = Result<T, GlassSystemError>;

/// A system should be the collection of the glasses and basic logic over them.
#[derive(Clone, Debug, Default, Eq)]
pub struct GlassSystem {
    system: Vec<Glass>,
}

impl Hash for GlassSystem {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        // Sort glass hashes for order-independent hashing
        let mut hashes: Vec<u64> = self
            .system
            .iter()
            .map(|glass| {
                let mut hasher = std::collections::hash_map::DefaultHasher::new();
                glass.hash(&mut hasher);
                hasher.finish()
            })
            .collect();

        hashes.sort_unstable();
        hashes.hash(state);
    }
}

impl PartialEq for GlassSystem {
    fn eq(&self, other: &Self) -> bool {
        if self.system.len() != other.system.len() {
            return false;
        }

        // Sort both systems by glass hash for comparison
        let mut self_sorted = self.system.clone();
        let mut other_sorted = other.system.clone();

        let sort_key = |glass: &Glass| {
            let mut hasher = std::collections::hash_map::DefaultHasher::new();
            glass.hash(&mut hasher);
            hasher.finish()
        };

        self_sorted.sort_by_key(sort_key);
        other_sorted.sort_by_key(sort_key);

        self_sorted == other_sorted
    }
}

impl GlassSystem {
    /// Some system is necessary, even if just an empty Vec.
    pub fn new(system: Vec<Glass>) -> Self {
        GlassSystem { system }
    }

    /// Empty glasses are not considered sorted, but for a sorted
    /// system some empty glasses remain for sure.
    pub fn is_solved(&self) -> bool {
        self.system.iter().all(|g| g.is_empty() || g.is_sorted())
    }

    /// Attempts to pour from one glass to another. "Try" refers to the fact that it can
    /// return an error.
    pub fn try_pour(&mut self, from_idx: usize, to_idx: usize) -> GlassSystemResult<()> {
        // Index check
        if from_idx >= self.system.len() || to_idx >= self.system.len() || from_idx == to_idx {
            return Err(GlassSystemError::InvalidGlassIndices(
                from_idx,
                to_idx,
                self.system.len(),
            ));
        }

        // Getting multiple mutable borrows from the same collection
        // "just like that" is unsafe in Rust as it cannot provide guarantees.
        // But splitting it into two prevents the possible issues that might arise
        // From mutating the same collection.
        let (source, destination) = match from_idx < to_idx {
            true => {
                let (front, back) = self.system.split_at_mut(to_idx);
                (&mut front[from_idx], &mut back[0])
            }
            false => {
                let (front, back) = self.system.split_at_mut(from_idx);
                (&mut back[0], &mut front[to_idx])
            }
        };

        Self::pour_from_to(source, destination, false)?;

        Ok(())
    }

    fn display_pour(source: &Glass, destination: &Glass) {
        println!();

        print!("╭───╮ ");
        print!("╭───╮ ");
        println!();

        for row in (0..=source.glass.len()).rev() {
            print!("│ {} │ ", source.glass[row]);
            print!("│ {} │ ", destination.glass[row]);
            println!();
        }

        print!("╰───╯ ");
        print!("╰───╯ ");
        println!();
    }

    fn pour_from_to(
        source: &mut Glass,
        destination: &mut Glass,
        debug: bool,
    ) -> GlassSystemResult<()> {
        if debug {
            println!("Before pouring:");
            Self::display_pour(source, destination);
            println!();
        }

        // A bit redunand (destination top cannot be empty and full) but whatever, it's clear at least.
        while (destination.top() == source.top() || destination.top() == Color::EMPTY)
            && !destination.is_full()
        {
            destination.push(source.pop())?;
        }

        if debug {
            println!("After pouring:");
            Self::display_pour(source, destination);
            println!("-----------------");
        }
        Ok(())
    }

    pub fn get_state(&self) -> &[Glass] {
        &self.system
    }

    pub fn print_system_state(&self) {
        for i in 0..self.system.len() {
            print!(" {i}    ");
        }
        println!();

        for _ in self.system.iter() {
            print!("╭───╮ ");
        }
        println!();

        for row in (0..self.system[0].glass.len()).rev() {
            for glass in self.system.iter() {
                print!("│ {} │ ", glass.glass[row]);
            }
            println!();
        }

        for _ in self.system.iter() {
            print!("╰───╯ ");
        }
        println!();
    }

    /// Gets all valid steps for a given node (or system)
    pub fn get_valid_steps(&self) -> Vec<Step> {
        let mut valid_steps = Vec::new();

        for (src_idx, source) in self.get_state().iter().enumerate() {
            if source.is_empty() || source.is_sorted() {
                continue;
            }

            for (dest_idx, destination) in self.get_state().iter().enumerate() {
                if destination.is_full() {
                    continue;
                }

                if dest_idx == src_idx {
                    continue;
                }

                if destination.is_empty() || source.top() == destination.top() {
                    valid_steps.push(Step {
                        source: src_idx,
                        destination: dest_idx,
                    })
                }
            }
        }
        valid_steps
    }
}
