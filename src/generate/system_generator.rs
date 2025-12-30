use rand::rng;
use rand::seq::SliceRandom;
use thiserror::Error;

use crate::game_elements::{
    color::Color,
    glass::{Glass, GlassError},
    glass_system::GlassSystem,
    GLASS_CAPACITY,
};

/// Custom error for the solver.
#[derive(Debug, Error)]
pub enum SystemGeneratorError {
    /// Derived from the GlassError if an issue happens on that level.
    #[error(transparent)]
    GlassError(#[from] GlassError),
}

pub type SystemGeneratorResult<T> = Result<T, SystemGeneratorError>;

/// Generate a random system.
/// Current rules:
/// - Two glasses are always completely empty
/// - All other glasses are always full
/// - Number of glasses = number of colors + 2
pub fn generate_random_system(no_colors: usize) -> SystemGeneratorResult<GlassSystem> {
    // TODO: Make it more flexible to allow experimentation with
    //       varying sizes and numbers including the glass capactiy
    //       that is currently a constant.
    let mut color_pool: Vec<Color> = Vec::with_capacity(GLASS_CAPACITY * no_colors);
    for color_id in 1..=no_colors {
        for _ in 0..GLASS_CAPACITY {
            color_pool.push(Color::new(color_id as u8));
        }
    }

    let mut rng = rng();
    color_pool.shuffle(&mut rng);

    let mut glasses = Vec::new();

    for glass_idx in 0..no_colors {
        let start_idx = glass_idx * GLASS_CAPACITY;
        let end_idx = start_idx + GLASS_CAPACITY;
        let glass_colors = color_pool[start_idx..end_idx].to_vec();
        glasses.push(Glass::from_colors(&glass_colors)?);
    }

    for _ in 0..2 {
        glasses.push(Glass::new());
    }

    Ok(GlassSystem::new(glasses))
}
