use clap::Parser;
use thiserror::Error;

use crate::{
    cli_arguments::parse_system::{parse_file_to_system, SystemParsingError},
    game_elements::{
        color_palette::{set_color_scheme, ColorScheme},
        glass_system::GlassSystem,
    },
    generate::system_generator::{generate_random_system_with_seed, SystemGeneratorError},
    solver::SolutionValueMode,
};

#[derive(Debug, Clone)]
pub enum SearchMethod {
    Bfs,
    Heuristic,
}

impl clap::ValueEnum for SearchMethod {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Bfs, Self::Heuristic]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Bfs => Some(clap::builder::PossibleValue::new("BFS")),
            Self::Heuristic => Some(clap::builder::PossibleValue::new("Heuristic")),
        }
    }
}

#[derive(Debug, Clone)]
pub enum HeuristicEvaluation {
    Constant,
    ColorCounting,
    ColorAlternation,
}

impl clap::ValueEnum for HeuristicEvaluation {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Constant, Self::ColorCounting, Self::ColorAlternation]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Constant => Some(clap::builder::PossibleValue::new("Constant")),
            Self::ColorCounting => Some(clap::builder::PossibleValue::new("ColorCounting")),
            Self::ColorAlternation => Some(clap::builder::PossibleValue::new("ColorAlternation")),
        }
    }
}

impl From<HeuristicEvaluation> for SolutionValueMode {
    fn from(value: HeuristicEvaluation) -> Self {
        match value {
            HeuristicEvaluation::Constant => SolutionValueMode::Constant,
            HeuristicEvaluation::ColorCounting => SolutionValueMode::ColorCount,
            HeuristicEvaluation::ColorAlternation => SolutionValueMode::AlternatingColors,
        }
    }
}

#[derive(Debug, Clone)]
enum ColorSchemeArg {
    Vibrant,
    Pastel,
    Muted,
    HighContrast,
}

impl clap::ValueEnum for ColorSchemeArg {
    fn value_variants<'a>() -> &'a [Self] {
        &[Self::Vibrant, Self::Pastel, Self::Muted, Self::HighContrast]
    }

    fn to_possible_value(&self) -> Option<clap::builder::PossibleValue> {
        match self {
            Self::Vibrant => Some(clap::builder::PossibleValue::new("Vibrant")),
            Self::Pastel => Some(clap::builder::PossibleValue::new("Pastel")),
            Self::Muted => Some(clap::builder::PossibleValue::new("Muted")),
            Self::HighContrast => Some(clap::builder::PossibleValue::new("HighContrast")),
        }
    }
}

impl From<ColorSchemeArg> for ColorScheme {
    fn from(arg: ColorSchemeArg) -> Self {
        match arg {
            ColorSchemeArg::Vibrant => ColorScheme::Vibrant,
            ColorSchemeArg::Pastel => ColorScheme::Pastel,
            ColorSchemeArg::Muted => ColorScheme::Muted,
            ColorSchemeArg::HighContrast => ColorScheme::HighContrast,
        }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// The search method: ["BFS", "Heuristic"]
    #[arg(short, long, default_value = "Heuristic")]
    pub search_method: SearchMethod,

    /// The search method: ["constant (dfs)", "color-count", "alternatin-colors"]
    #[arg(short, long, default_value = "ColorCounting")]
    pub heuristic_evaluation: HeuristicEvaluation,

    /// Random seed for the system genration
    #[arg(short, long, default_value = "42")]
    pub random_seed: u64,

    /// Number of colors
    #[arg(short, long)]
    pub number_of_colors: usize,

    /// Color scheme: ["Vibrant", "Pastel", "Muted", "HighContrast"]
    #[arg(short = 'c', long, default_value = "Vibrant")]
    color_scheme: ColorSchemeArg,

    /// Size of the glasses (uniform)
    #[arg(short = 's', long, default_value = "4")]
    glass_size: usize,

    /// System to solve file path
    #[arg(short = 'p', long, default_value = None)]
    input_system: Option<String>,
}

/// Custom error for the low-level stuff.
#[derive(Debug, Error)]
pub enum ArgumentsError {
    #[error(transparent)]
    SystemParsingError(#[from] SystemParsingError),

    #[error(transparent)]
    SystemGeneratorError(#[from] SystemGeneratorError),

    #[error("Invalid glass size: {0}. It must be at least 2.")]
    InvalidGlassSize(usize),

    #[error("Invalid number of colors: {0}. It must be at least 2")]
    InvalidNumberOfColors(usize),
}

impl Args {
    pub fn process_arguments() -> Result<(Self, GlassSystem), ArgumentsError> {
        let args = Args::parse();

        if args.glass_size < 2 {
            return Err(ArgumentsError::InvalidGlassSize(args.glass_size));
        }

        if args.number_of_colors < 2 {
            return Err(ArgumentsError::InvalidNumberOfColors(args.number_of_colors));
        }

        set_color_scheme(args.color_scheme.clone().into());
        let system_to_solve = match &args.input_system {
            None => generate_random_system_with_seed(
                args.number_of_colors,
                args.random_seed,
                args.glass_size,
            )?,
            Some(path) => parse_file_to_system(path.clone())?,
        };

        Ok((args, system_to_solve))
    }
}
