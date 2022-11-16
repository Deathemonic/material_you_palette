//! A collection of functions for blending in HCT and CAM16.
//!
//! All formulas here are derived from the original Blend sources in <https://github.com/material-foundation/material-color-utilities>
use crate::hct::cam16::Cam16;
use crate::hct::Hct;
use crate::utils::color::lstar_from_argb;
use crate::utils::math::{difference_degrees, rotation_direction, sanitize_degrees_double};

/// Blend the design color's HCT hue towards the key color's HCT hue, in a way
/// that leaves the original color recognizable and recognizably shifted
/// towards the key color.
///
/// # Arguments
///
/// * `design_color`: ARGB representation of an arbitrary color.
/// * `source_color`: ARGB representation of the main theme color.
///
/// # Returns
///
/// * The design color with a hue shifted towards the system's color, a
///   slightly warmer/cooler variant of the design color's hue.
pub fn harmonize(design_color: [u8; 4], source_color: [u8; 4]) -> [u8; 4] {
    let from_hct = Hct::from_int(design_color);
    let to_hct = Hct::from_int(source_color);
    let difference_degrees = difference_degrees(from_hct.hue(), to_hct.hue());
    let rotation_degrees = (difference_degrees * 0.5).min(15.0);
    let output_hue = sanitize_degrees_double(
        from_hct.hue() + rotation_degrees * rotation_direction(from_hct.hue(), to_hct.hue()),
    );
    Hct::from(output_hue, from_hct.chroma(), from_hct.tone()).to_int()
}

/// Blends hue from one color into another. The chroma and tone of the original color are
/// maintained.
///
/// # Arguments
///
/// * `from`: ARGB representation of color
/// * `to`: ARGB representation of color
/// * `amount`: how much blending to perform; 0.0 >= and <= 1.0
///
/// # Returns
///
/// * A shade of `from`, with a hue blended towards ARGB `to`. Chroma and
///   tone are constant.
pub fn hct_hue(from: [u8; 4], to: [u8; 4], amount: f64) -> [u8; 4] {
    let ucs = cam16ucs(from, to, amount);
    let ucs_cam = Cam16::from_argb(ucs);
    let from_cam = Cam16::from_argb(from);
    let blended = Hct::from(ucs_cam.hue(), from_cam.chroma(), lstar_from_argb(from));
    blended.to_int()
}

/// Blend in CAM16-UCS space.
///
/// # Arguments
///
/// * `from`: ARGB representation of color
/// * `to`: ARGB representation of color
/// * `amount`: how much blending to perform; 0.0 >= and <= 1.0
///
/// # Returns
///
/// * A shade of `from`, blended towards ARGB `to`. Hue, chroma, and tone
///   will change.
pub fn cam16ucs(from: [u8; 4], to: [u8; 4], amount: f64) -> [u8; 4] {
    let from_cam = Cam16::from_argb(from);
    let to_cam = Cam16::from_argb(to);
    let from_j = from_cam.jstar();
    let from_a = from_cam.astar();
    let from_b = from_cam.bstar();
    let to_j = to_cam.jstar();
    let to_a = to_cam.astar();
    let to_b = to_cam.bstar();
    let jstar = from_j + (to_j - from_j) * amount;
    let astar = from_a + (to_a - from_a) * amount;
    let bstar = from_b + (to_b - from_b) * amount;
    Cam16::from_jch(jstar, astar, bstar).to_int()
}

#[cfg(test)]
mod tests {
    use crate::blend::harmonize;

    const RED: [u8; 4] = [255, 255, 0, 0];
    const BLUE: [u8; 4] = [255, 0, 0, 255];
    const GREEN: [u8; 4] = [255, 0, 255, 0];
    const YELLOW: [u8; 4] = [255, 255, 255, 0];

    #[test]
    fn harmonize_red_to_blue() {
        let val = harmonize(RED, BLUE);
        assert_eq!(val, [255, 251, 0, 87]);
    }

    #[test]
    fn harmonize_red_to_green() {
        let val = harmonize(RED, GREEN);
        assert_eq!(val, [255, 216, 86, 0]);
    }

    #[test]
    fn harmonize_red_to_yellow() {
        let val = harmonize(RED, YELLOW);
        assert_eq!(val, [255, 216, 86, 0]);
    }

    #[test]
    fn harmonize_blue_to_green() {
        let val = harmonize(BLUE, GREEN);
        assert_eq!(val, [255, 0, 71, 163]);
    }

    #[test]
    fn harmonize_blue_to_red() {
        let val = harmonize(BLUE, RED);
        assert_eq!(val, [255, 87, 0, 220]);
    }

    #[test]
    fn harmonize_blue_to_yellow() {
      let val = harmonize(BLUE, YELLOW);
        assert_eq!(val, [255, 0, 71, 163]);
    }

    #[test]
    fn harmonize_green_to_blue() {
        let val = harmonize(GREEN, BLUE);
        assert_eq!(val, [255, 0, 252, 148]);
    }

    #[test]
    fn harmonize_green_to_red() {
        let val = harmonize(GREEN, RED);
        assert_eq!(val, [255, 177, 240, 0]);
    }

    #[test]
    fn harmonize_green_to_yellow() {
        let val = harmonize(GREEN, YELLOW);
        assert_eq!(val, [255, 177, 240, 0]);
    }

    #[test]
    fn harmonize_yellow_to_blue() {
        let val = harmonize(YELLOW, BLUE);
        assert_eq!(val, [255, 235, 255, 186]);
    }

    #[test]
    fn harmonize_yellow_to_green() {
        let val = harmonize(YELLOW, GREEN);
        assert_eq!(val, [255, 235, 255, 186]);
    }

    #[test]
    fn harmonize_yellow_to_red() {
        let val = harmonize(YELLOW, RED);
        assert_eq!(val, [255, 255, 246, 227]);
    }
}
