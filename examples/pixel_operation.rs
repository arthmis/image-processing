extern crate image_processing;
extern crate image;

use image_processing::pixel_operations::*;
use image_processing::window::display_image;

use image::ConvertBuffer;

fn main() {
    let img = image::open("images/gray-london-bridge.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();
    let width = 1000;
    let height = 1000;
    // let inverted_img = invert_grayscale(&img);
    // display_image("inverted", &inverted_img.convert(), 1000, 1000);
    // let inverted_img = invert_grayscale_mut(img);
    // display_image("inverted", &inverted_img.convert(), 1000, 1000);
    let thresholded_image = threshold(&img, 125);
    display_image("thresholded", &thresholded_image.convert(), width, height);
    let thresholded_image = threshold_mut(img, 125);
    display_image("thresholded mut", &thresholded_image.convert(), width, height);

}
