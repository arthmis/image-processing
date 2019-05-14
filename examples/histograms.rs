extern crate image;
extern crate image_processing;

use image::ConvertBuffer;
use image_processing::statistics::histogram::*;
use image_processing::window::display_image;

fn main() {
    let my_img = image::open("images/london-bridge.jpg")
        .expect("Image not found")
        .to_luma();

    let img_hist = gray_histogram(&my_img);
    let histogram_img = convert_to_image(1000, 500, &img_hist);
    display_image("histogram", &histogram_img.convert(), 800, 801);
}
