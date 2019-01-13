use image_processing::point_operations::invert_grayscale;

use image::ConvertBuffer;
use image_processing::window::display_image;

fn main() {
    let img = image::open("images/gray-london-bridge.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();
    let inverted_img = invert_grayscale(&img);
    display_image("inverted", inverted_img.convert(), 1000, 1000);

}
