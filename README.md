# PyMonet (WIP)

**A direct quote from the original source material's readme:**

>Algorithms and utilities that power the Material Design 3 (M3) color system, including choosing theme colors from images and creating tones of colors; all in a new color space.

This is a rendition / continuation of the [material_color_utilities_rs](https://github.com/alphaqu/material-color-utilities-rs) library which itself is based on the original [material-color-utilities](https://github.com/material-foundation/material-color-utilities) project. In many cases, I have "plagiarized" code from the Rust version with minimal modifications outside of restructuring directories and files / filenames, correcting typos, cleaning up / adding tests, adding comments and documentation where necessary and in general just doing housekeeping. In other cases, I have filled in the missing pieces from the original library and added modules I found useful.

See the original [README](https://github.com/material-foundation/material-color-utilities#readme) for more information about the M3 system.

To learn more about what the Material You palette is, you can read more about how Google developed a whole new color-space called [HCT](https://material.io/blog/science-of-color-design).

## Credits

I'm getting this out of the way now. Big thanks to [alphaqu](https://github.com/alphaqu) for their work on the Rust implementation. That proved to be a HUGE head start. Also, big thank you to the [Material Foundation](https://github.com/material-foundation) for providing the original material *(pun intended)* upon which mine and alphaqu's work is based. Between the two they provided more than enough foundation for this project.

## Getting started

```rust
use material_you_palette::{
    utils::theme::Theme,
    utils::string::argb_from_hex,
    utils::string::hex_from_argb,
    scheme::Role,
};

fn main() {
  // One liner to create an entire palette - both dark and light modes - from a single color.
  let theme = Theme::from_source_color(argb_from_hex("#4c5f9e"));
  // `theme` should now be a complete set of colors observably similar or related to #4c5f9e.

  // We can now pluck colors out of the theme by specifying which mode (scheme) - light/dark
  // - we want and then by the "role" for that color. Learn more about color roles here:
  // https://m3.material.io/styles/color/the-color-system/color-roles

  // Here we get two colors, both of which are "argb" values in [u8; 4] format.
  let primary_button_bg = theme.schemes.dark.primary;
  let primary_button_fg = theme.schemes.dark.on_primary;

  // Or we can automatically convert them to HEX rgb values with an included utility.
  let primary_button_hex_bg = hex_from_argb(theme.schemes.dark.primary);
  let primary_button_hex_fg = hex_from_argb(theme.schemes.dark.on_primary);

  // Additionally, the roles have been enumerated and can be pulled by those.
  let background = theme.schemes.light[&Role::Background];
  // Again, we get "argb" as [u8; 4]. We can use the `hex_from_argb` function here also.
  let surface = hex_from_argb(theme.schemes.light[&Role::Surface]);
}
```

## What's left TODO?

- [ ] Add the ability to load an image file to identify the "key color" for a palette.
- [ ] Additional testing
- [ ] Optimizations (I know there is a lot of room for improvements)
- [ ] Documentation improvements
- [ ] Incorporate serde for optionally getting JSON for the palette

## Project Goals

The primary goal for this project is to provide a simple library in Rust to produce a palette that is compatible with the Material You color system as detailed by the Material Foundation. The focus of this library is specifically on the colors, roles and other associated concepts of the palette.

The main goals:

1. Produce a palette using the HCT + L colorspace defined by Google for Material Design.
2. The palette should be produced from a single color supplied by the user.
    - The color may be supplied directly or manually
    - The color may be "plucked" from a supplied image file
3. The produced palette should be available in as many necessary or useful formats.

What this project isn't:

- This is not a an application that will generate a set of theme files for any particular application. This library should be used by such an application.
- Although there is an ability to load image files, the only purpose for that is to select a primary or key color from the image file on which to base the HCT palette. This library should not be setting wallpapers or otherwise altering system themes / appearance. This library should be used by such an application.
- This library is not intended to be a stand-alone application, thus the term library being used. So there should be no command-line arguments to be processed, no `fn main` or `main.rs` files included in the repo. Everything in the crate should stem from the `lib.rs` only.

## Contributing

Contributions are always welcome. Please see the [contributions](./CONTRIBUTING.md) note for more information.
