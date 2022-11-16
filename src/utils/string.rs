//! A utility library for converting to and from hex color strings.
//!
//! This library makes the assumption that all hex strings supplied and returned
//! adhere to CSS standards for hex color strings. This means that the library
//! supports short-code colors (3 characters like #FFF for white), standard RGB
//! color strings (6 characters like #FF0000 for red), and RGBA color strings to
//! support an alpha channel (8 characters like #C6C6C680 for a gray that is
//! partly translucent).
//!
//! NOTE: Any alpha channel in hex colors supplied and returned is expected to
//! be the last value in the string. This is compliant with the standard form
//! used in CSS / HTML.
use super::color::{alpha_from_argb, blue_from_argb, green_from_argb, red_from_argb};
use hex::FromHex;

/// Returns a hex RGB string representation of an ARGB numeric.
///
/// # Arguments
///
/// * `argb`: ARGB representation of a color.
///
/// # Returns
///
/// * Hex string representing color, ex. #ff0000 for red.
pub fn hex_from_argb(argb: [u8; 4]) -> String {
    let a: u8 = alpha_from_argb(argb);
    let r = red_from_argb(argb);
    let g = green_from_argb(argb);
    let b = blue_from_argb(argb);
    let hex_value = if a < 255 {
        hex::encode([r, g, b, a])
    } else {
        hex::encode([r, g, b])
    };
    String::from("#") + &hex_value
}

/// Returns an ARGB numeric representation of a hex RGB(A) string
///
/// # Arguments
///
/// * `hex`: String representing color as hex code. Accepts strings with or without leading #, and string representing the color using 3, 6, or 8 hex characters.
///
/// # Returns
///
/// * ARGB representation of color in a [u8; 4] package.
pub fn argb_from_hex(hex: String) -> [u8; 4] {
    let trimmed_hex = hex.replace('#', "");
    let mut a: u8 = 255;
    let r: u8;
    let g: u8;
    let b: u8;

    match trimmed_hex.len() {
        3 => {
            r = <[u8; 1]>::from_hex(trimmed_hex[0..1].repeat(2)).unwrap()[0];
            g = <[u8; 1]>::from_hex(trimmed_hex[1..2].repeat(2)).unwrap()[0];
            b = <[u8; 1]>::from_hex(trimmed_hex[2..].repeat(2)).unwrap()[0];
        }
        6 => {
            [r, g, b] = <[u8; 3]>::from_hex(trimmed_hex).unwrap();
        }
        8 => {
            [r, g, b, a] = <[u8; 4]>::from_hex(trimmed_hex).unwrap();
        }
        _ => panic!("Invalid hex color string supplied."),
    }
    [a, r, g, b]
}

#[cfg(test)]
mod tests {
    use crate::utils::string::{argb_from_hex, hex_from_argb};

    #[test]
    fn get_argb_from_hex() {
        let argb_one = argb_from_hex(String::from("#770099"));
        assert_eq!(argb_one[0], 255);
        assert_eq!(argb_one[1], 119);
        assert_eq!(argb_one[2], 0);
        assert_eq!(argb_one[3], 153);
    }

    #[test]
    fn get_argb_from_hex_three() {
        let argb_two = argb_from_hex(String::from("#709"));
        assert_eq!(argb_two[0], 255);
        assert_eq!(argb_two[1], 119);
        assert_eq!(argb_two[2], 0);
        assert_eq!(argb_two[3], 153);
    }

    #[test]
    #[should_panic]
    fn test_argb_from_hex_panic() {
        let argb = argb_from_hex(String::from("#12345"));
        assert_eq!(argb[0], 12);
    }

    #[test]
    fn get_hex_from_argb() {
        let hex = hex_from_argb([255, 119, 0, 153]);
        assert_eq!(hex, String::from("#770099"));
    }

    #[test]
    fn get_argb_from_hex_alpha() {
        let argb = argb_from_hex(String::from("#77009980"));
        assert_eq!(argb[0], 128);
        assert_eq!(argb[1], 119);
        assert_eq!(argb[2], 0);
        assert_eq!(argb[3], 153);
    }

    #[test]
    fn get_hex_alpha_from_argb() {
        let hex = hex_from_argb([128, 119, 0, 153]);
        assert_eq!(hex, String::from("#77009980"));
    }
}
