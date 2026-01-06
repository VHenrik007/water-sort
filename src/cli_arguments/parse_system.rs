use std::{collections::HashMap, fs, io::Error as IOError, num::ParseIntError};
use thiserror::Error;

use crate::game_elements::{color::Color, glass::Glass, glass_system::GlassSystem};

/// Custom error for the low-level stuff.
#[derive(Debug, Error)]
pub enum SystemParsingError {
    #[error(transparent)]
    IOError(#[from] IOError),

    #[error(transparent)]
    ParseIntError(#[from] ParseIntError),

    #[error("The provided file is empty!")]
    EmptyFile,

    #[error("Inconsistent glass size in line compared to the first line!")]
    InconsistentGlassSize,

    #[error("The number of color {0} appears more times than the glass size!")]
    TooManyColorOccurrences(Color),

    #[error("The number of color {0} appears less times than the glass size!")]
    TooFewColorOccurrences(Color),
}

pub fn parse_file_to_system(path: String) -> Result<GlassSystem, SystemParsingError> {
    let file_content = fs::read_to_string(path)?;
    let mut glasses = Vec::new();
    let mut color_counter: HashMap<Color, i32> = HashMap::new();

    let mut lines = file_content.lines();
    let Some(first_line) = lines.next() else {
        return Err(SystemParsingError::EmptyFile);
    };

    let mut colors = get_colors_from_line(first_line)?;

    for color in &colors {
        color_counter.entry(color.clone()).and_modify(|v| *v += 1).or_insert(1);
    }

    let glass_size = colors.len();
    glasses.push(Glass::from_colors(&mut colors));

    for line in lines {
        let mut colors = get_colors_from_line(line)?;
        for color in &colors {
            color_counter.entry(color.clone()).and_modify(|v| *v += 1).or_insert(1);
        }

        if glass_size != colors.len() {
            return Err(SystemParsingError::InconsistentGlassSize);
        }

        glasses.push(Glass::from_colors(&mut colors));
    }

    for (color, num) in color_counter {
        if num > glass_size as i32 {
            return Err(SystemParsingError::TooManyColorOccurrences(color))
        }
        if num < glass_size as i32 {
            return Err(SystemParsingError::TooFewColorOccurrences(color))
        }
    }

    glasses.extend_from_slice(
        &[Glass::new(glass_size), Glass::new(glass_size)]
    );


    Ok(GlassSystem::new(glasses))
}

fn get_colors_from_line(line: &str) -> Result<Vec<Color>, SystemParsingError> {
    line.split(';')
        .map(|cs| {
            let parsed_color = cs.parse()?;
            Ok(Color::new(parsed_color))
        })
        .collect::<Result<Vec<_>, SystemParsingError>>()
}
