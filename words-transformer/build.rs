#[macro_use]
extern crate slash_formatter;
extern crate image;

use std::fs;

use image::GenericImageView;

const LOGO_INPUT_PATH: &str = concat_with_file_separator!("assets", "icons", "logo.png");
const LOGO_OUTPUT_PATH: &str = concat_with_file_separator!("src", "logo.rs");

fn main() {
    // Load the logo image (in common formats) and turn it to RGBA to store in a Rust code file as a constant.
    let logo_image = image::open(LOGO_INPUT_PATH).unwrap();

    let logo_width = logo_image.width();
    let logo_height = logo_image.height();
    let logo_vec_rgba = logo_image.to_rgba8().to_vec();

    let logo_rs = format!(
        "/* DON'T EDIT. This file is auto-generated. */\npub(crate) const WIDTH: u32 = {};\npub(crate) const HEIGHT: u32 = {};\npub(crate) const DATA: &[u8] = &{:?};",
        logo_width,
        logo_height,
        logo_vec_rgba
    );

    fs::write(LOGO_OUTPUT_PATH, logo_rs).unwrap();
}
