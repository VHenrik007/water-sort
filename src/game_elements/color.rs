use colored::{Color as TextColor, *};
use std::fmt;

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
            // Cycle through available colors based on ID
            // TODO: Make it more granular somehow using some range or idk.
            let display_str = match self.id % 8 {
                1 => "■".red(),
                2 => "■".green(),
                3 => "■".blue(),
                4 => "■".yellow(),
                5 => "■".cyan(),
                6 => "■".magenta(),
                7 => "■".color(TextColor::TrueColor { r: 255, g: 0, b: 0 }),
                8 => "■".bright_green(),
                9 => "■".bright_blue(),
                10 => "■".bright_yellow(),
                11 => "■".bright_cyan(),
                0 => "■".bright_green(),
                _ => unreachable!(),
            };
            write!(f, "{}", display_str)
        }
    }
}
