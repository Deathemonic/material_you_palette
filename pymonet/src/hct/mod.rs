//! A color system built using CAM16 hue and chroma, and L* from L*a*b*.
//!
//! Using L* creates a link between the color system, contrast, and thus
//! accessibility. Contrast ratio depends on relative luminance, or Y in the XYZ
//! color space. L*, or perceptual luminance can be calculated from Y.
//!
//! Unlike Y, L* is linear to human perception, allowing trivial creation of
//! accurate color tones.
//!
//! Unlike contrast ratio, measuring contrast in L* is linear, and simple to
//! calculate. A difference of 40 in HCT tone guarantees a contrast ratio >= 3.0,
//! and a difference of 50 guarantees a contrast ratio >= 4.5.
use crate::hct::cam16::Cam16;
use crate::utils::color::lstar_from_argb;

pub mod cam16;
pub mod hct_solver;
pub mod viewing_conditions;

#[derive(Default)]
pub struct Hct {
    internal_hue: f64,
    internal_chroma: f64,
    internal_tone: f64,
    argb: [u8; 4],
}

/// HCT, hue, chroma, and tone. A color system that provides a perceptually
/// accurate color measurement system that can also accurately render what colors
/// will appear as in different lighting environments.
impl Hct {
    /// Create an HCT color from hue, chroma, and tone.
    ///
    /// # Arguments
    ///
    /// * `hue`: hue 0 <= hue < 360; invalid values are corrected.
    /// * `chrome`: chroma 0 <= chroma < ?; Informally, colorfulness. The color returned may be lower than the requested chroma. Chroma has a different maximum for any given hue and tone.
    /// * `tone`: tone 0 <= tone <= 100; invalid values are corrected.
    ///
    /// # Returns
    /// * HCT representation of a color in default viewing conditions.
    pub fn from(hue: f64, chroma: f64, tone: f64) -> Hct {
        let mut htc = Hct::default();
        htc.set_internal_state(hct_solver::solve_to_int(hue, chroma, tone));
        htc
    }

    /// Create an HCT color from a color.
    ///
    /// # Arguments
    ///
    /// * `argb`: ARGB representation of a color.
    ///
    /// # Returns
    /// * HCT representation of a color in default viewing conditions
    pub fn from_int(argb: [u8; 4]) -> Hct {
        let mut htc = Hct::default();
        htc.set_internal_state(argb);
        htc
    }

    /// Getter for `hue`
    ///
    /// # Returns
    /// * The current hue value as a float
    pub fn hue(&self) -> f64 {
        self.internal_hue
    }

    /// Getter for `chroma`
    ///
    /// # Returns
    /// * The current chroma value as a float
    pub fn chroma(&self) -> f64 {
        self.internal_chroma
    }

    /// Getter for `tone`
    ///
    /// # Returns
    /// * The current tone value as a float
    pub fn tone(&self) -> f64 {
        self.internal_tone
    }

    /// Getter for the calculated HCT color
    ///
    /// # Returns
    /// * The current color value as an ARGB value
    pub fn to_int(&self) -> [u8; 4] {
        self.argb
    }

    /// Set the hue of this color. Chroma may decrease because chroma has a different maximum for any
    /// given hue and tone.
    ///
    /// # Arguments
    ///
    /// * `hue`: 0 <= newHue < 360; invalid values are corrected.
    pub fn set_hue(&mut self, hue: f64) {
        self.set_internal_state(hct_solver::solve_to_int(
            hue,
            self.internal_chroma,
            self.internal_tone,
        ))
    }

    /// Set the chroma of this color. Chroma may decrease because chroma has a different maximum for
    /// any given hue and tone.
    ///
    /// # Arguments
    ///
    /// * `chroma`: 0 <= newChroma < ?
    pub fn set_chroma(&mut self, chroma: f64) {
        self.set_internal_state(hct_solver::solve_to_int(
            self.internal_hue,
            chroma,
            self.internal_tone,
        ))
    }

    /// Set the tone of this color. Chroma may decrease because chroma has a different maximum for any
    /// given hue and tone.
    ///
    /// # Arguments
    ///
    /// * `tone`: 0 <= newTone <= 100; invalid valids are corrected.
    pub fn set_tone(&mut self, tone: f64) {
        self.set_internal_state(hct_solver::solve_to_int(
            self.internal_hue,
            self.internal_chroma,
            tone,
        ))
    }

    fn set_internal_state(&mut self, argb: [u8; 4]) {
        self.argb = argb;
        let cam = Cam16::from_argb(argb);
        self.internal_hue = cam.hue();
        self.internal_chroma = cam.chroma();
        self.internal_tone = lstar_from_argb(argb);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hct::viewing_conditions::ViewingConditions;
    use crate::utils::color::y_from_lstar;
    use assert_approx_eq::assert_approx_eq;

    const BLACK: [u8; 4] = [0xff, 0x00, 0x00, 0x00];
    const WHITE: [u8; 4] = [0xff, 0xff, 0xff, 0xff];
    const RED: [u8; 4] = [0xff, 0xff, 0x00, 0x00];
    const GREEN: [u8; 4] = [0xff, 0x00, 0xff, 0x00];
    const BLUE: [u8; 4] = [0xff, 0x00, 0x00, 0xff];
    // Figure out how to test MIDGRAY
    // const MIDGRAY: [u8; 4] = [0xff, 0x77, 0x77, 0x77];

    #[test]
    fn conversions_are_reflexive() {
        let cam = Cam16::from_argb(RED);
        let conditions = ViewingConditions::default();
        println!("{conditions:#?}");
        let color = cam.viewed(conditions);
        assert_eq!(color, RED)
    }

    #[test]
    fn colors() {
        assert_approx_eq!(18.418, y_from_lstar(50.0), 0.001);
        assert_approx_eq!(0.0, y_from_lstar(0.0), 0.001);
        assert_approx_eq!(100.0, y_from_lstar(100.0), 0.001);
    }

    #[test]
    fn cam_red() {
        let cam = Cam16::from_argb(RED);
        assert_approx_eq!(46.445, cam.j(), 0.001);
        assert_approx_eq!(113.357, cam.chroma(), 0.001);
        assert_approx_eq!(27.408, cam.hue(), 0.001);
        assert_approx_eq!(89.494, cam.m(), 0.001);
        assert_approx_eq!(91.889, cam.s(), 0.001);
        assert_approx_eq!(105.988, cam.q(), 0.001);
    }

    #[test]
    fn cam_green() {
        let cam = Cam16::from_argb(GREEN);
        assert_approx_eq!(79.331, cam.j(), 0.001);
        assert_approx_eq!(108.410, cam.chroma(), 0.001);
        assert_approx_eq!(142.139, cam.hue(), 0.001);
        assert_approx_eq!(85.587, cam.m(), 0.001);
        assert_approx_eq!(78.604, cam.s(), 0.001);
        assert_approx_eq!(138.520, cam.q(), 0.001);
    }

    #[test]
    fn cam_blue() {
        let cam = Cam16::from_argb(BLUE);
        assert_approx_eq!(25.465, cam.j(), 0.001);
        assert_approx_eq!(87.230, cam.chroma(), 0.001);
        assert_approx_eq!(282.788, cam.hue(), 0.001);
        assert_approx_eq!(68.867, cam.m(), 0.001);
        assert_approx_eq!(93.674, cam.s(), 0.001);
        assert_approx_eq!(78.481, cam.q(), 0.001);
    }

    #[test]
    fn cam_black() {
        let cam = Cam16::from_argb(BLACK);
        assert_approx_eq!(0.0, cam.j(), 0.001);
        assert_approx_eq!(0.0, cam.chroma(), 0.001);
        assert_approx_eq!(0.0, cam.hue(), 0.001);
        assert_approx_eq!(0.0, cam.m(), 0.001);
        assert_approx_eq!(0.0, cam.s(), 0.001);
        assert_approx_eq!(0.0, cam.q(), 0.001);
    }

    #[test]
    fn cam_white() {
        let cam = Cam16::from_argb(WHITE);
        assert_approx_eq!(100.0, cam.j(), 0.001);
        assert_approx_eq!(2.869, cam.chroma(), 0.001);
        assert_approx_eq!(209.492, cam.hue(), 0.001);
        assert_approx_eq!(2.265, cam.m(), 0.001);
        assert_approx_eq!(12.068, cam.s(), 0.001);
        assert_approx_eq!(155.521, cam.q(), 0.001);
    }

    #[test]
    fn gamut_map_colors() {
        fn gamut_map_test(color_to_test: [u8; 4]) {
            let cam = Cam16::from_argb(color_to_test);
            let color = Hct::from(cam.hue(), cam.chroma(), lstar_from_argb(color_to_test)).to_int();
            assert_eq!(color_to_test, color);
        }

        gamut_map_test(RED);
        gamut_map_test(GREEN);
        gamut_map_test(BLUE);
        gamut_map_test(WHITE);
        gamut_map_test(BLACK);
    }
}
