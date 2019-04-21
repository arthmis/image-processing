extern crate image;
extern crate image_processing;

use image_processing::pixel_operations::auto_contrast;
use image_processing::window::display_multiple_images;
use image::ConvertBuffer;

pub fn main() {

    let img = image::open("images/empire-state-building-black-white-dark.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();

    let auto_contrast_img = auto_contrast(&img);
    let (width, height) = (500, 500);

    display_multiple_images(
        &[
            "original",
            "auto contrast",
        ],
        &[
            &img.convert(),
            &auto_contrast_img.convert(),
        ],
        width,
        height
    );
}