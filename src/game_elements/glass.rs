use std::hash::Hash;
use thiserror::Error;

use crate::game_elements::color::Color;
use crate::game_elements::GLASS_CAPACITY;

/// A single glass in the game
#[derive(Debug, Clone, Copy)]
pub struct Glass {
    /// Contains color IDs. Each glass has the same length.
    pub glass: [Color; GLASS_CAPACITY + 1],
    /// The index of the top element. If the glass is full
    /// it's `GLASS_CAPACITY`, and then -1 each time we pour out.
    pub top: usize,
}

impl PartialEq for Glass {
    fn eq(&self, other: &Self) -> bool {
        for color_idx in 1..GLASS_CAPACITY + 1 {
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
    #[error("Invalid number of constructor parameters: {0}. Expected: {GLASS_CAPACITY}")]
    InvalidConstructorColorLength(usize),
    #[error("The glass is full.")]
    FullGlass,
}

impl Default for Glass {
    fn default() -> Self {
        Self::new()
    }
}

impl Glass {
    /// Default color with full empty colors.
    pub fn new() -> Self {
        Glass {
            top: 0,
            glass: [Color::EMPTY; GLASS_CAPACITY + 1],
        }
    }

    /// Left->right in array is bottom -> top in glass (stack).
    pub fn from_colors(colors: &[Color]) -> GlassResult<Self> {
        if colors.len() != GLASS_CAPACITY {
            return Err(GlassError::InvalidConstructorColorLength(colors.len()));
        }

        Ok(Glass {
            top: colors.len(),
            glass: {
                let mut array = [Color::EMPTY; GLASS_CAPACITY + 1];
                array[1..].copy_from_slice(&colors[0..GLASS_CAPACITY]);
                array
            },
        })
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
        self.top == GLASS_CAPACITY
    }

    /// If true, cannot pour from it.
    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    /// If true, the glass is solved.
    pub fn is_sorted(&self) -> bool {
        // The loop below could be true for full-empty glasses.
        if self.top != GLASS_CAPACITY {
            return false;
        }

        for i in 1..GLASS_CAPACITY + 1 {
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
