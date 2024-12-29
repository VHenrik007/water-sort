use thiserror::Error;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::glass::GLASS_CAPACITY;
use crate::glass::color::Color;

static GLASS_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);

#[derive(Debug, Clone, Copy)]
pub struct Glass {
    pub glass: [Color; GLASS_CAPACITY+1],
    pub top: usize,
    pub id: usize,
}

pub type GlassResult<T> = Result<T, GlassError>;

#[derive(Debug, Error)]
pub enum GlassError {
    #[error("Invalid number of constructor parameters: {0}. Expected: {GLASS_CAPACITY}")]
    InvalidConstructorColorLength(usize),
    #[error("The glass is full.")]
    FullGlass
}


impl Glass{
    pub fn new() -> Self {
        Glass {
            top: 0,
            glass: [Color::EMPTY; GLASS_CAPACITY+1],
            id: GLASS_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        }
    }

    pub fn from_colors(colors: &[Color]) -> GlassResult<Self> {
        if colors.len() != GLASS_CAPACITY {
            return Err(GlassError::InvalidConstructorColorLength(colors.len()));
        }

        Ok(Glass {
            top: colors.len(),
            glass: {
                let mut array = [Color::EMPTY; GLASS_CAPACITY+1];
                array[1..].copy_from_slice(&colors[0..GLASS_CAPACITY]);
                array
            },
            id: GLASS_ID_COUNTER.fetch_add(1, Ordering::Relaxed),
        })
    }

    pub fn top(&self) -> Color {
        self.glass[self.top]
    }

    pub fn pop(&mut self) -> Color {
        let current_color = self.glass[self.top];

        if self.top != 0 {
            self.glass[self.top] = Color::EMPTY;
            self.top = self.top - 1;
        }

        current_color
    }

    pub fn push(&mut self, color: Color) -> GlassResult<()> {
        if self.top == GLASS_CAPACITY {
            return Err(GlassError::FullGlass)
        }

        self.top = self.top + 1;
        self.glass[self.top] = color;

        Ok(())
    }

    pub fn is_full(&self) -> bool {
        self.top == GLASS_CAPACITY
    }

    pub fn is_empty(&self) -> bool {
        self.top == 0
    }

    pub fn is_sorted(&self) -> bool {
        if self.top != GLASS_CAPACITY{
            return false
        }

        for i in 1..GLASS_CAPACITY + 1 {
            if self.glass[i] != self.glass[1] {
                return false
            }
        }

        true
    }

    pub fn print_state(&self) {
        println!("╭───╮");
        for color in self.glass.iter().rev() {
            println!("│ {} │", color);
        }
        println!("╰───╯");
    }

}
