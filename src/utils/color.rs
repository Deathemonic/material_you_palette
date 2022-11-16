//! Collection of commonly used color calculations and transformations
use crate::utils::math::matrix_multiply;

/// Maps calculation values from sRGB color space to XYZ
pub const SRGB_TO_XYZ: [[f64; 3]; 3] = [
    [0.41233895, 0.35762064, 0.18051042],
    [0.2126, 0.7152, 0.0722],
    [0.01932141, 0.11916382, 0.95034478],
];

/// Maps calculation values from XYZ color space to sRGB
pub const XYZ_TO_SRGB: [[f64; 3]; 3] = [
    [
        3.2413774792388685,
        -1.5376652402851851,
        -0.49885366846268053,
    ],
    [-0.9691452513005321, 1.8758853451067872, 0.04156585616912061],
    [
        0.05562093689691305,
        -0.20395524564742123,
        1.0571799111220335,
    ],
];

/// A fixed shade of white; white on a sunny day.
pub const WHITE_POINT_D65: [f64; 3] = [95.047, 100.0, 108.883];

/// Converts a color from RGB components to ARGB format
///
/// # Arguments
///
/// * `rgb`: A color value mapped to distinct RGB values
///
/// # Returns
///
/// * An ARGB color value mapped to distinct ARGB values
pub fn argb_from_rgb(rgb: [u8; 3]) -> [u8; 4] {
    [255, rgb[0], rgb[1], rgb[2]]
}

/// Converts a color from linear RGB components to ARGB format
///
/// # Arguments
///
/// * `linrgb`: Color value in distinct linear RGB values
///
/// # Returns
/// * An ARGB color value mapped to distinct ARGB values
pub fn argb_from_linrgb(linrgb: [f64; 3]) -> [u8; 4] {
    let r = delinearized(linrgb[0]);
    let g = delinearized(linrgb[1]);
    let b = delinearized(linrgb[2]);
    argb_from_rgb([r, g, b])
}

/// Returns the alpha component of a color in ARGB format
///
/// # Arguments
///
/// * `argb`: A color value mapped to distinct ARGB values
///
/// # Returns
///
/// * The alpha channel value ranging from 0 to 255
pub fn alpha_from_argb(argb: [u8; 4]) -> u8 {
    argb[0]
}

/// Returns the red component of a color in ARGB format
///
/// # Arguments
///
/// * `argb`: A color value mapped to distinct ARGB values
///
/// # Returns
///
/// * The red channel value ranging from 0 to 255
pub fn red_from_argb(argb: [u8; 4]) -> u8 {
    argb[1]
}

/// Returns the green component of a color in ARGB format
///
/// # Arguments
///
/// * `argb`: A color value mapped to distinct ARGB values
///
/// # Returns
///
/// * The green channel value ranging from 0 to 255
pub fn green_from_argb(argb: [u8; 4]) -> u8 {
    argb[2]
}

/// Returns the blue component of a color in ARGB format
///
/// # Arguments
///
/// * `argb`: A color value mapped to distinct ARGB values
///
/// # Returns
///
/// * The blue channel value ranging from 0 to 255
pub fn blue_from_argb(argb: [u8; 4]) -> u8 {
    argb[3]
}

/// Returns whether a color in ARGB format is opaque
///
/// # Arguments
///
/// * `argb`: A color value mapped to distinct ARGB values
///
/// # Returns
///
/// * true if the alpha channel is 255
pub fn is_opaque(argb: [u8; 4]) -> bool {
    alpha_from_argb(argb) == 255
}

/// Converts a color from XYZ to ARGB
///
/// # Arguments
///
/// * `xyz`: A color value mapped to XYZ color space
///
/// # Returns
///
/// * An ARGB equivalent of the supplied color
pub fn argb_from_xyz(xyz: [f64; 3]) -> [u8; 4] {
    let rgb = matrix_multiply(xyz, XYZ_TO_SRGB);
    let r = delinearized(rgb[0]);
    let g = delinearized(rgb[1]);
    let b = delinearized(rgb[2]);
    argb_from_rgb([r, g, b])
}

/// Converts a color from ARGB to XYZ
///
/// # Arguments
///
/// * `argb`: A color value mapped to sRGB color space
///
/// # Returns
///
/// * An XYZ equivalent of the supplied color
pub fn xyz_from_argb(argb: [u8; 4]) -> [f64; 3] {
    let r = linearized(argb[1]);
    let g = linearized(argb[2]);
    let b = linearized(argb[3]);
    matrix_multiply([r, g, b], SRGB_TO_XYZ)
}

/// Converts a color from L*a*b* color space to ARGB
///
/// # Arguments
///
/// * `l`: Lightness value of the color
/// * `a`: Red/Green value of the color
/// * `b`: Blue/Yellow value of the color
///
/// # Returns
///
/// * An ARGB equivalent of the supplied color
pub fn argb_from_lab(l: f64, a: f64, b: f64) -> [u8; 4] {
    let fy = (l + 16.0) / 116.0;
    let fx = a / 500.0 + fy;
    let fz = fy - b / 200.0;
    let x = lab_invf(fx) * WHITE_POINT_D65[0];
    let y = lab_invf(fy) * WHITE_POINT_D65[1];
    let z = lab_invf(fz) * WHITE_POINT_D65[2];
    argb_from_xyz([x, y, z])
}

/// Converts a color from ARGB color space to L*a*b*
///
/// # Arguments
///
/// * `argb`: A color value mapped to sRGB color space
///
/// # Returns
///
/// * An L*a*b* equivalent of the supplied color
pub fn lab_from_argb(argb: [u8; 4]) -> [f64; 3] {
    let [x, y, z] = xyz_from_argb(argb);
    let fx = lab_f(x / WHITE_POINT_D65[0]);
    let fy = lab_f(y / WHITE_POINT_D65[1]);
    let fz = lab_f(z / WHITE_POINT_D65[2]);
    let l = 116.0 * fy - 16.0;
    let a = 500.0 * (fx - fy);
    let b = 200.0 * (fy - fz);
    [l, a, b]
}

/// Converts an L* value to an ARGB representation.
///
/// # Arguments
///
/// * `lstar`: The Lightness value of an L*a*b* color
///
/// # Returns
///
/// * ARGB representation of grayscale color with lightness matching L*
pub fn argb_from_lstar(lstar: f64) -> [u8; 4] {
    let y = y_from_lstar(lstar);
    let w = delinearized(y);
    argb_from_rgb([w, w, w])
}

/// Computes the L* value of a color in ARGB representation.
///
/// # Arguments
///
/// * `argb`: A color value mapped to sRGB color space
///
/// # Returns
///
/// * L*, from L*a*b*, coordinate of the color
pub fn lstar_from_argb(argb: [u8; 4]) -> f64 {
    let y = xyz_from_argb(argb)[1];
    116.0 * lab_f(y / 100.0) - 16.0
}

/// Converts an L* value to a Y value.
///
/// L* in L*a*b* and Y in XYZ measure the same quantity, luminance. L* measures
/// perceptual luminance, a linear scale. Y in XYZ measures relative luminance,
/// a logarithmic scale.
///
/// # Arguments
///
/// * `lstar`: The Lightness value of an L*a*b* color
///
/// # Returns
///
/// * The value of Y from the XYZ color space that corresponds to the L* value
pub fn y_from_lstar(lstar: f64) -> f64 {
    100.0 * lab_invf((lstar + 16.0) / 116.0)
}

/// Linearizes an RGB component.
///
/// # Arguments
///
/// * `rgb_comp`: RGB channel component to normalize
///
/// # Returns
///
/// * 0.0 <= output <= 100.0, color channel converted to linear RGB space
pub fn linearized(rgb_comp: u8) -> f64 {
    let normalized = rgb_comp as f64 / 255.0;
    if normalized <= 0.040449936 {
        normalized / 12.92 * 100.0
    } else {
        ((normalized + 0.055) / 1.055).powf(2.4) * 100.0
    }
}

/// Delinearizes an RGB component.
///
/// # Arguments
///
/// * `rgb_comp`: RGB channel component to normalize
///
/// # Returns
///
/// * 0 <= output <= 255, color channel converted to regular RGB space
pub fn delinearized(rgb_comp: f64) -> u8 {
    let normalized = rgb_comp / 100.0;
    let delinearized = if normalized <= 0.0031308 {
        normalized * 12.92
    } else {
        1.055 * normalized.powf(1.0 / 2.4) - 0.055
    };
    (delinearized * 255.0).round().clamp(0.0, 255.0) as u8
}

/// Returns the standard white point
///
/// # Returns
///
/// * A fixed shade of white; white on a sunny day
pub fn white_point_d65() -> [f64; 3] {
    WHITE_POINT_D65
}

/// Returns a perceived luminance value of `t`
///
/// Used to identify the perceived luminance of a supplied value from the ARGB
/// color space. This is needed to convert RGB colors to L*a*b* colors.
///
/// # Arguments
///
/// * `t`: The value of R,G or B to convert
///
/// # Returns
///
/// * The perceived luminance of `t`.
fn lab_f(t: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    if t > e {
        t.powf(1.0 / 3.0)
    } else {
        (kappa * t + 16.0) / 116.0
    }
}

/// Returns an inverted perceived luminance value of `ft`
///
/// Used to convert a color from L*a*b* color space to RGB color space.
///
/// # Arguments
///
/// * `ft`: The luminance value of L*, a*, or b*
///
/// # Returns
///
/// * The base R, G or B value to then multiply against the standard brightness
///   of WHITE_POINT_D65.
fn lab_invf(ft: f64) -> f64 {
    let e = 216.0 / 24389.0;
    let kappa = 24389.0 / 27.0;
    let ft3 = ft * ft * ft;
    if ft3 > e {
        ft3
    } else {
        (116.0 * ft - 16.0) / kappa
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::color::{
        alpha_from_argb, argb_from_lab, argb_from_linrgb, argb_from_lstar, argb_from_rgb,
        argb_from_xyz, blue_from_argb, delinearized, green_from_argb, is_opaque, lab_from_argb,
        linearized, lstar_from_argb, red_from_argb, white_point_d65, xyz_from_argb, y_from_lstar,
        WHITE_POINT_D65,
    };

    #[test]
    fn test_argb_from_rgb() {
        let argb = argb_from_rgb([119, 0, 153]);
        assert_eq!(argb[0], 255);
        assert_eq!(argb[1], 119);
        assert_eq!(argb[2], 0);
        assert_eq!(argb[3], 153);
    }

    #[test]
    fn test_argb_from_linrgb() {
        let argb = argb_from_linrgb([18.4474994500441, 18.4474994500441, 18.4474994500441]);
        assert_eq!(argb[0], 255);
        assert_eq!(argb[1], 119);
        assert_eq!(argb[2], 119);
        assert_eq!(argb[3], 119);
    }

    #[test]
    fn test_alpha_from_argb() {
        let alpha = alpha_from_argb([255, 119, 0, 153]);
        assert_eq!(alpha, 255);
    }

    #[test]
    fn test_red_from_argb() {
        let red = red_from_argb([255, 119, 0, 153]);
        assert_eq!(red, 119);
    }

    #[test]
    fn test_green_from_argb() {
        let green = green_from_argb([255, 119, 0, 153]);
        assert_eq!(green, 0);
    }

    #[test]
    fn test_blue_from_argb() {
        let blue = blue_from_argb([255, 119, 0, 153]);
        assert_eq!(blue, 153);
    }

    #[test]
    fn test_is_opaque() {
        let fixed_argb_one = [255, 119, 0, 153];
        let fixed_argb_two = [160, 72, 102, 190];
        let is = is_opaque(fixed_argb_one);
        let isnot = is_opaque(fixed_argb_two);
        assert_eq!(is, true);
        assert_eq!(isnot, false);
    }

    #[test]
    fn test_argb_from_xyz() {
        let xyz = [13.356723824257475, 6.221846121142539, 30.629358478049];
        let argb = argb_from_xyz(xyz);
        assert_eq!(argb[0], 255);
        assert_eq!(argb[1], 119);
        assert_eq!(argb[2], 0);
        assert_eq!(argb[3], 153);
    }

    #[test]
    fn test_xyz_from_argb() {
        let xyz = xyz_from_argb([255, 119, 0, 153]);
        assert_eq!(xyz[0], 13.356723824257475);
        assert_eq!(xyz[1], 6.221846121142539);
        assert_eq!(xyz[2], 30.629358478049);
    }

    #[test]
    fn test_argb_from_lab() {
        let argb = argb_from_lab(29.965403607253286, 61.82367536548383, -51.794952267087055);
        assert_eq!(argb[0], 255);
        assert_eq!(argb[1], 119);
        assert_eq!(argb[2], 0);
        assert_eq!(argb[3], 153);
    }

    #[test]
    fn test_lab_from_argb() {
        let lab = lab_from_argb([255, 119, 0, 153]);
        assert_eq!(lab[0], 29.965403607253286);
        assert_eq!(lab[1], 61.82367536548383);
        assert_eq!(lab[2], -51.794952267087055);
    }

    #[test]
    fn test_argb_from_lstar() {
        let argb = argb_from_lstar(29.965403607253286);
        assert_eq!(argb[0], 255);
        assert_eq!(argb[1], 71);
        assert_eq!(argb[2], 71);
        assert_eq!(argb[3], 71);
    }

    #[test]
    fn test_lstar_from_argb() {
        let lstar = lstar_from_argb([255, 119, 0, 153]);
        assert_eq!(lstar, 29.965403607253286);
    }

    #[test]
    fn test_y_from_lstar() {
        let y = y_from_lstar(29.965403607253286);
        assert_eq!(y, 6.221846121142538);
    }

    #[test]
    fn test_linearized() {
        let lin = linearized(119);
        assert_eq!(lin, 18.4474994500441);
    }

    #[test]
    fn test_delinearized() {
        let delin = delinearized(18.4474994500441);
        assert_eq!(delin, 119);
    }

    #[test]
    fn test_white_point_d65() {
        let wp = white_point_d65();
        assert_eq!(wp, WHITE_POINT_D65);
    }
}
