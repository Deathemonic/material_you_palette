use crate::palettes::{core::CorePalette, tonal::TonalPalette};
use crate::scheme::Scheme;

/// Custom color used to pair with a theme
#[derive(Debug)]
pub struct CustomColor {
    pub value: [u8; 4],
    pub name: String,
    pub blend: bool,
}

/// Color group
#[derive(Debug)]
pub struct ColorGroup {
    pub color: [u8; 4],
    pub on_color: [u8; 4],
    pub color_container: [u8; 4],
    pub on_color_container: [u8; 4],
}

/// Custom Color Group
#[derive(Debug)]
pub struct CustomColorGroup {
    pub color: CustomColor,
    pub value: [u8; 4],
    pub light: ColorGroup,
    pub dark: ColorGroup,
}

/// Collection of color schemes based of the palette source color
#[derive(Debug, Clone)]
pub struct Schemes {
    pub light: Scheme,
    pub dark: Scheme,
}

/// A collection of palettes..
#[derive(Debug, Clone)]
pub struct Palettes {
    pub primary: TonalPalette,
    pub secondary: TonalPalette,
    pub tertiary: TonalPalette,
    pub neutral: TonalPalette,
    pub neutral_variant: TonalPalette,
    pub error: TonalPalette,
}

/// Theme object
///
/// Holds the data specific to a theme based on a source color
#[derive(Debug, Clone)]
pub struct Theme {
    pub source: [u8; 4],
    pub schemes: Schemes,
    pub palettes: Palettes,
}

impl Theme {
    /// Generate a theme from a source color
    ///
    /// @param source Source color
    /// @param customColors Array of custom colors
    /// @return Theme object
    pub fn from_source_color(source: [u8; 4]) -> Theme {
        let mut palette = CorePalette::new(source, false);
        let light = Scheme::light_from_core_palette(&mut palette);
        let dark = Scheme::dark_from_core_palette(&mut palette);
        let schemes: Schemes = Schemes { light, dark };
        let primary = palette.a1;
        let secondary = palette.a2;
        let tertiary = palette.a3;
        let neutral = palette.n1;
        let neutral_variant = palette.n2;
        let error = palette.error;
        let palettes: Palettes = Palettes {
            primary,
            secondary,
            tertiary,
            neutral,
            neutral_variant,
            error,
        };
        Theme {
            source,
            schemes,
            palettes,
        }
    }

    pub fn from_source_colors(sources: [[u8; 4]; 3]) -> Theme {
        let mut palette = CorePalette::new(sources[0], true);
        let light = Scheme::light_from_core_palette(&mut palette);
        let dark = Scheme::dark_from_core_palette(&mut palette);
        let schemes: Schemes = Schemes { light, dark };
        let primary = palette.a1;
        let secondary = palette.a2;
        let tertiary = palette.a3;
        let neutral = palette.n1;
        let neutral_variant = palette.n2;
        let error = palette.error;
        let palettes: Palettes = Palettes {
            primary,
            secondary,
            tertiary,
            neutral,
            neutral_variant,
            error,
        };
        Theme {
            source: sources[0],
            schemes,
            palettes,
        }
    }
}
