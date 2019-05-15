
extern crate image;
extern crate image_processing;

use image_processing::pixel_operations::*;
// use image_processing::statistics::histogram::*;
use image_processing::window::display_multiple_images;

use image::ConvertBuffer;

fn main() {
    let mut img = image::open("images/england-hampton-court-palace.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();
    let base_img = img.clone();

    exposure_compensation_mut(&mut img, -1.0);
    let (width, height) = (500, 500);

    display_multiple_images(
        &["original", "compensated"],
        &[&base_img.convert(), &img.convert()],
        width,
        height,
    );
}