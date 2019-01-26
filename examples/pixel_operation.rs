extern crate image_processing;
extern crate image;

use image_processing::pixel_operations::*;
use image_processing::window::display_multiple_images;

use image::ConvertBuffer;

fn main() {
    let img = image::open("images/gray-london-bridge.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();
    let width = 500;
    let height = 500;
    let inverted_img = invert_grayscale(&img);
    // display_image("inverted", &inverted_img.convert(), 1000, 1000);
    // let inverted_img = invert_grayscale_mut(img.clone());
    // display_image("inverted", &inverted_img.convert(), 1000, 1000);
    let thresholded_image = threshold(&img, 125);
    // display_image("thresholded", &thresholded_image.convert(), width, height);
    // let thresholded_image = threshold_mut(img.clone(), 125);
    // display_image("thresholded mut", &thresholded_image.convert(), width, height);
    // display_image("regular image", &img.convert(), width, height);
    // let brightness_image = brightness(&img, -50);
    // display_image("increase brightness", &brightness_image.convert(), width, height);
    let contrast_image = contrast(&img, 50);
    // display_image("increase contrast", &contrast_image.convert(), width, height);
    display_multiple_images(&["base", "inverted", "thresholded", "contrast"], &[&img.convert(), &inverted_img.convert(), &thresholded_image.convert(), &contrast_image.convert()], width, height);

}
