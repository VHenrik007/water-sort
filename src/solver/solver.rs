use thiserror::Error;

use crate::glass::{glass::{Glass, GlassError}, GLASS_CAPACITY, color::Color};

#[derive(Debug, Error)]
pub enum SolverError {
    #[error("Invalid glass indices: from:{0}, to:{1}, system length: {2}")]
    InvalidGlassIndices(usize, usize, usize),
    #[error(transparent)]
    GlassError(#[from] GlassError),
}

pub type SolverResult<T> = Result<T, SolverError>;

pub struct Solver {
    system: Vec<Glass>,
}

impl Solver {
    pub fn new(system: Vec<Glass>) -> Self {
        Solver { system }
    }

    pub fn is_solved(&self) -> bool {
        self.system.iter().all(|g| g.is_empty() || g.is_sorted())
    }

    pub fn try_pour(&mut self, from_idx: usize, to_idx: usize) -> SolverResult<()> {
        if from_idx >= self.system.len() || to_idx >= self.system.len() || from_idx == to_idx {
            return Err(SolverError::InvalidGlassIndices(from_idx, to_idx, self.system.len()));
        }

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


    fn pour_from_to(source: &mut Glass, destination: &mut Glass, debug:bool) -> SolverResult<()> {
        if debug {
            println!("Before pouring:");
            Self::display_pour(source, destination, source.id, destination.id);
            println!();
        }

        while (destination.top() == source.top() || destination.top() == Color::EMPTY) && !destination.is_full() {
            destination.push(source.pop())?;
        }

        if debug{
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
}