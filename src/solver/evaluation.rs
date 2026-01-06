use std::collections::HashSet;

use crate::game_elements::{color::Color, glass_system::GlassSystem};

/// Lower is better.
/// A metric that determines how "far off" we are from a solution
pub type SolutionValue = u32;

#[allow(dead_code)]
pub enum SolutionValueMode {
    /// No evaluation.
    Constant,
    /// Counts the number of different colors in each glass
    /// This means that the more homogeneus glasses are,
    /// the more promising that state is.
    ColorCount,
    /// Beyond the color count, it also might matter how mixed
    /// a glass is. Even if it contains just 2 colors, if they
    /// alternate, that probably needs more steps to tangle out.
    AlternatingColors,
}

/// Calculates the value of a system.
pub fn solution_value(system: &GlassSystem, mode: &SolutionValueMode) -> SolutionValue {
    let empty_penalty: SolutionValue = 1;
    let value = match mode {
        SolutionValueMode::Constant => 1,
        SolutionValueMode::ColorCount => color_count_metric(system),
        SolutionValueMode::AlternatingColors => alternating_color_metric(system),
    };
    value + empty_glass_penalty(system, empty_penalty)
}

/// Being empty means a bit more than being solved.
fn empty_glass_penalty(system: &GlassSystem, reward: SolutionValue) -> SolutionValue {
    let mut value: SolutionValue = 0;
    for glass in system.get_state() {
        if glass.is_empty() {
            value += reward;
        }
    }

    value
}

fn color_count_metric(system: &GlassSystem) -> SolutionValue {
    let mut value: SolutionValue = 0;
    let mut different_colors = HashSet::new();
    for glass in system.get_state() {
        for color in &glass.glass {
            different_colors.insert(color);
        }
        value += different_colors.len() as SolutionValue;
    }

    value
}

fn alternating_color_metric(system: &GlassSystem) -> SolutionValue {
    let mut value: SolutionValue = 0;
    for glass in system.get_state() {
        let mut last_color = Color::EMPTY;
        for color in &glass.glass {
            if last_color != *color {
                value += 1;
                last_color = color.clone();
            }
        }
    }

    value
}
