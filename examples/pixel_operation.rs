extern crate image;
extern crate image_processing;

use image_processing::pixel_operations::*;
use image_processing::statistics::histogram::*;
use image_processing::window::display_multiple_images;

use image::ConvertBuffer;

fn main() {
    let img = image::open("images/england-hampton-court-palace.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();
    let next_img = image::open("images/empire-state-building-black-white.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();
    let width = 500;
    let height = 500;
    let histogram_specification = match_piecewise_linear_histogram(&img, &next_img);

    display_multiple_images(
        &[
            "base",
            "histogram specification",
            "target image",
        ],
        &[
            &img.convert(),
            &histogram_specification.convert(),
            &next_img.convert(),
        ],
        width,
        height,
    );

    img.save("images/base.jpg").unwrap();
    histogram_specification.save("images/matched_histogram.jpg").unwrap();
    next_img.save("images/reference_image.jpg").unwrap();
}
