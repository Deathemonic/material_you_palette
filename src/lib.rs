//! # m3_colors
//!
//! ##### A direct quote from the original source material's readme:
//!
//! >Algorithms and utilities that power the Material Design 3 (M3) color system, including choosing theme colors from images and creating tones of colors; all in a new color space.
//!
//! This is a rendition / continuation of the [material_color_utilities_rs](https://github.com/alphaqu/material-color-utilities-rs) library which itself is based on the original [material-color-utilities](https://github.com/material-foundation/material-color-utilities) project. In many cases, I have "plagiarized" code from the original Rust package with minimal modifications outside of restructuring directories and files / filenames, correcting typos, cleaning up / adding tests, adding comments and documentation where necessary and in general just doing housekeeping. In other cases, I have filled in the missing pieces from the original library and added modules I found as useful.
//!
//! See the original [README](https://github.com/material-foundation/material-color-utilities#readme) for more information about the M3 system.
//! ## Getting started
pub mod blend;
pub mod hct;
pub mod palettes;
// pub mod quantize;
pub mod scheme;
pub mod score;
pub mod utils;
