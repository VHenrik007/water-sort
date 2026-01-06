use std::{hash::Hash, vec};
use thiserror::Error;

use crate::game_elements::color::Color;

/// A single glass in the game
#[derive(Debug, Clone)]
pub struct Glass {
    /// Contains color IDs. Each glass has the same length.
    pub glass: Vec<Color>,
    /// Capacity of the glass, the number of most colors that it can store. (The vector is this + 1 for the empty case)
    pub capacity: usize,
    /// The index of the top element. If the glass is full
    /// it's the total number of colors, and then -1 each unit of liquid we pour out.
    pub top: usize,
}

impl PartialEq for Glass {
    fn eq(&self, other: &Self) -> bool {
        for color_idx in 1..self.capacity + 1 {
            if self.glass[color_idx] != other.glass[color_idx] {
                return false;
            }
        }
        true
    }
}

impl Hash for Glass {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.glass.hash(state);
        self.top.hash(state);
    }
}

impl Eq for Glass {}

/// Just to make things easier.
pub type GlassResult<T> = Result<T, GlassError>;

/// Custom error for the low-level stuff.
#[derive(Debug, Error)]
pub enum GlassError {
    #[error("Invalid number of constructor parameters: {0}.")]
    InvalidConstructorColorLength(usize),
    #[error("The glass is full.")]
    FullGlass,
}

impl Glass {
    /// Default color with full empty colors.
    pub fn new(glass_size: usize) -> Self {
        let glass = vec![Color::EMPTY; glass_size + 1];
        Glass { top: 0, glass, capacity: glass_size }
    }

    /// Left->right in array is bottom -> top in glass (stack).
    pub fn from_colors(glass: &mut Vec<Color>) -> Self {
        let mut padded_glass = vec![Color::EMPTY];
        padded_glass.append(glass);
        padded_glass.shrink_to(1);
        let capacity = padded_glass.len() - 1;
        Glass {
            top: capacity,
            glass: padded_glass,
            capacity,
        }
    }

    /// Top color.
    pub fn top(&self) -> Color {
        self.glass[self.top]
    }

    /// Pours the top-most color only. Same-color pouring
    /// is enforced in a different logic.
    /// TODO: If empty, probably should have an error as well?
    ///       That would make it consistent with push.
    pub fn pop(&mut self) -> Color {
        let current_color = self.glass[self.top];

        if self.top != 0 {
            self.glass[self.top] = Color::EMPTY;
            self.top -= 1;
        }

        current_color
    }

    /// If not full, put the desired color on top.
    /// The color-validity is enforced on another level.
    pub fn push(&mut self, color: Color) -> GlassResult<()> {
        if self.is_full() {
            return Err(GlassError::FullGlass);
        }

        self.top += 1;
        self.glass[self.top] = color;

        Ok(())
    }

    /// If true, cannot pour into it.
    pub fn is_full(&self) -> bool {
        self.top == self.capacity
    }

    /// If true, cannot pour from it.
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// If true, the glass is solved.
    pub fn is_sorted(&self) -> bool {
        // The loop below could be true for full-empty glasses.
        if self.top != self.capacity {
            return false;
        }

        for i in 1..self.capacity + 1 {
            if self.glass[i] != self.glass[1] {
                return false;
            }
        }

        true
    }

    /// Utility function.
    pub fn print_state(&self) {
        println!("╭───╮");
        for color in self.glass.iter().rev() {
            println!("│ {} │", color);
        }
        println!("╰───╯");
    }
}
