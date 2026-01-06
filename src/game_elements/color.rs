use colored::*;
use std::fmt;

use crate::game_elements::color_palette::colorize_by_id;

/// A single unit that occupies a slot in a glass.
#[derive(Debug, PartialEq, Clone, Copy, Hash, Eq)]
pub struct Color {
    id: u8,
}

impl Color {
    pub const EMPTY: Color = Color { id: 0 };

    pub fn new(id: u8) -> Self {
        Color { id }
    }

    pub fn is_empty(&self) -> bool {
        self.id == 0
    }

    pub fn id(&self) -> u8 {
        self.id
    }
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_empty() {
            write!(f, "{}", "□".white())
        } else {
            let display_str = colorize_by_id(self.id, "■");
            write!(f, "{}", display_str)
        }
    }
}
