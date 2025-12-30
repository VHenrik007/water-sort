use thiserror::Error;

use crate::game_elements::{
    color::Color,
    glass::{Glass, GlassError},
    step::Step,
    GLASS_CAPACITY,
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
#[derive(Hash, Clone, Default)]
pub struct GlassSystem {
    system: Vec<Glass>,
}

impl PartialEq for GlassSystem {
    /// System equality currently is checked "order-pairwise" instead of universally.
    /// This means that equality is false even if they have the same glasses but in different
    /// order. TODO: Refine this.
    fn eq(&self, other: &Self) -> bool {
        let my_state = self.get_state();
        let other_state = other.get_state();
        if my_state.len() != other_state.len() {
            return false;
        }

        for glass_idx in 0..my_state.len() {
            if my_state[glass_idx] != other_state[glass_idx] {
                return false;
            }
        }

        true
    }
}

impl Eq for GlassSystem {}

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

    fn display_pour(source: &Glass, destination: &Glass, source_idx: usize, dest_idx: usize) {
        print!(" {source_idx}    ");
        print!(" {dest_idx}    ");
        println!();

        print!("╭───╮ ");
        print!("╭───╮ ");
        println!();

        for row in (0..=GLASS_CAPACITY).rev() {
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
            Self::display_pour(source, destination, source.id, destination.id);
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
            Self::display_pour(source, destination, source.id, destination.id);
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

        for row in (0..=GLASS_CAPACITY).rev() {
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
            if source.is_empty() {
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
                        source: source.id,
                        destination: destination.id,
                    })
                }
            }
        }
        valid_steps
    }
}
