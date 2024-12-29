use colored::*;
use std::fmt;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Color{
    EMPTY,
    RED,
    GREEN,
    BLUE,
    YELLOW,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let display_str = match self {
            Color::EMPTY => "□".white(),
            Color::RED => "■".red(),
            Color::GREEN => "■".green(),
            Color::BLUE => "■".blue(),
            Color::YELLOW => "■".yellow(),
        };
        write!(f, "{}", display_str)
    }
}