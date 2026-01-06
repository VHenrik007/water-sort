use colored::*;
use std::sync::{LazyLock, Mutex};

/// Color schemes for different visual styles
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub enum ColorScheme {
    /// Saturated colors
    #[default]
    Vibrant,
    /// Softer colors
    Pastel,
    /// Earthy tones
    Muted,
    /// High contrast
    HighContrast,
}

/// RGB color representation
#[derive(Debug, Clone, Copy)]
struct Rgb {
    r: u8,
    g: u8,
    b: u8,
}

impl Rgb {
    /// Convert HSL to RGB using the standard algorithm
    /// h: hue in degrees (0-360)
    /// s: saturation (0.0-1.0)
    /// l: lightness (0.0-1.0)
    fn from_hsl(h: f64, s: f64, l: f64) -> Self {
        let chroma = (1.0 - (2.0 * l - 1.0).abs()) * s;

        // Intermediate value for calculating the second-largest RGB component
        // based on the hue's position within its 60-degree segment.
        let second_component = chroma * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());

        // Match value: adjusts RGB values to account for lightness.
        // This shifts the color toward white (positive) or black (negative)
        let lightness_adjustment = l - chroma / 2.0;

        // Determine RGB values based on which 60-degree hue segment we're in
        // Each segment has a different pattern of which component is dominant
        let (r_temp, g_temp, b_temp) = if h < 60.0 {
            (chroma, second_component, 0.0)
        } else if h < 120.0 {
            (second_component, chroma, 0.0)
        } else if h < 180.0 {
            (0.0, chroma, second_component)
        } else if h < 240.0 {
            (0.0, second_component, chroma)
        } else if h < 300.0 {
            (second_component, 0.0, chroma)
        } else {
            (chroma, 0.0, second_component)
        };

        // Apply lightness adjustment and convert to 0-255 range
        let r_val = (r_temp + lightness_adjustment) * 255.0;
        let g_val = (g_temp + lightness_adjustment) * 255.0;
        let b_val = (b_temp + lightness_adjustment) * 255.0;

        Rgb {
            r: r_val.clamp(0., 255.) as u8,
            g: g_val.clamp(0., 255.) as u8,
            b: b_val.clamp(0., 255.) as u8,
        }
    }

    /// Apply this RGB color to a string using true color
    fn colorize(&self, text: &str) -> ColoredString {
        text.truecolor(self.r, self.g, self.b)
    }
}

/// Creates colors that are evenly distributed around the color wheel
fn generate_palette(scheme: ColorScheme, count: usize) -> Vec<Rgb> {
    let (base_saturation, base_lightness, hue_offset): (f64, f64, f64) = match scheme {
        ColorScheme::Vibrant => (0.85, 0.55, 0.0),
        ColorScheme::Pastel => (0.5, 0.75, 0.0),
        ColorScheme::Muted => (0.4, 0.5, 0.0),
        ColorScheme::HighContrast => (0.9, 0.5, 0.0),
    };

    let mut colors = Vec::with_capacity(count);

    // Pretty neat stuff I just learned!
    // https://en.wikipedia.org/wiki/Golden_angle
    let golden_angle = 137.508;

    for i in 0..count {
        let hue = (hue_offset + (i as f64 * golden_angle)) % 360.0;

        // Vary saturation and lightness slightly for better distinction
        let saturation_variation: f64 = if i % 3 == 0 { 0.05 } else { -0.05 };
        let lightness_variation: f64 = if i % 4 == 0 { 0.05 } else { -0.05 };

        let saturation_raw = base_saturation + saturation_variation;
        let saturation = saturation_raw.clamp(0.3, 1.0);

        let lightness_raw = base_lightness + lightness_variation;
        let lightness = lightness_raw.clamp(0.3, 0.8);

        colors.push(Rgb::from_hsl(hue, saturation, lightness));
    }

    colors
}

/// Color palette manager
pub struct ColorPalette {
    scheme: ColorScheme,
    colors: Vec<Rgb>,
}

impl ColorPalette {
    /// Create a new color palette with the specified scheme
    /// Supports up to 30 colors, but this is arbitrary
    /// TODO: Some config in the future?
    pub fn new(scheme: ColorScheme) -> Self {
        // Generate 30 colors to support large puzzles
        let colors = generate_palette(scheme, 30);
        ColorPalette { scheme, colors }
    }

    /// Get the color for a given color ID (1-based, 0 is empty)
    fn get_color(&self, color_id: u8) -> Option<&Rgb> {
        if color_id == 0 {
            return None;
        }

        let index = ((color_id - 1) as usize) % self.colors.len();
        Some(&self.colors[index])
    }

    /// Get the current color scheme
    pub fn scheme(&self) -> ColorScheme {
        self.scheme
    }
}

/// Global color scheme
static COLOR_SCHEME: LazyLock<Mutex<ColorScheme>> =
    LazyLock::new(|| Mutex::new(ColorScheme::default()));

/// Set the global color scheme
pub fn set_color_scheme(scheme: ColorScheme) {
    if let Ok(mut current_scheme) = COLOR_SCHEME.lock() {
        *current_scheme = scheme;
    }
}

/// Get the current color scheme
pub fn get_color_scheme() -> ColorScheme {
    COLOR_SCHEME.lock().map(|s| *s).unwrap_or_default()
}

/// Get a color palette instance for the current scheme
pub fn get_color_palette() -> ColorPalette {
    ColorPalette::new(get_color_scheme())
}

/// Colorize a string based on color ID using the current color scheme
pub fn colorize_by_id(color_id: u8, text: &str) -> ColoredString {
    if color_id == 0 {
        return text.white();
    }

    let palette = get_color_palette();
    if let Some(rgb) = palette.get_color(color_id) {
        rgb.colorize(text)
    } else {
        text.white()
    }
}
