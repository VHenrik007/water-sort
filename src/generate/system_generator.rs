use rand::rngs::StdRng;
use rand::seq::SliceRandom;
use rand::Rng;
use rand::SeedableRng;
use thiserror::Error;

use crate::game_elements::{
    color::Color,
    glass::{Glass, GlassError},
    glass_system::GlassSystem,
};

/// Custom error for the solver.
#[derive(Debug, Error)]
pub enum SystemGeneratorError {
    /// Derived from the GlassError if an issue happens on that level.
    #[error(transparent)]
    GlassError(#[from] GlassError),
}

pub type SystemGeneratorResult<T> = Result<T, SystemGeneratorError>;

/// Generate a random system with a seed for deterministic generation.
/// Current rules:
/// - Two glasses are always completely empty
/// - All other glasses are always full
/// - Number of glasses = number of colors + 2
pub fn generate_random_system_with_seed(
    no_colors: usize,
    seed: u64,
    glass_size: usize,
) -> SystemGeneratorResult<GlassSystem> {
    let mut color_pool: Vec<Color> = Vec::with_capacity(glass_size * no_colors);
    for color_id in 1..=no_colors {
        for _ in 0..glass_size {
            color_pool.push(Color::new(color_id as u8));
        }
    }

    // Create a seeded RNG
    let mut rng = StdRng::seed_from_u64(seed);
    color_pool.shuffle(&mut rng);

    let mut glasses = Vec::new();

    for glass_idx in 0..no_colors {
        let start_idx = glass_idx * glass_size;
        let end_idx = start_idx + glass_size;
        let mut glass_colors = color_pool[start_idx..end_idx].to_vec();
        glasses.push(Glass::from_colors(&mut glass_colors));
    }

    // NOTE: Allowing only one empty glass
    //       makes things much faster for smaller problems.
    let no_empty_glasses = 2;
    for _ in 0..no_empty_glasses {
        glasses.push(Glass::new(glass_size));
    }

    Ok(GlassSystem::new(glasses))
}

/// Convenience function that uses the current time or a random seed.
pub fn generate_random_system(
    no_colors: usize,
    glass_size: usize,
) -> SystemGeneratorResult<GlassSystem> {
    let seed = rand::rng().random();
    generate_random_system_with_seed(no_colors, seed, glass_size)
}
