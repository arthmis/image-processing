extern crate image;
extern crate image_processing;

use image::{GrayImage};
use image_processing::statistics::histogram::*;
use image_processing::window::display_image;
use image::ConvertBuffer;

fn main() {
    let my_img = image::open("images/london-bridge.jpg")
       .expect("Image not found").to_luma();

    // let some = integral_image(&img); 

    // let my_img = GrayImage::from_raw(
    //     4, 
    //     3, 
    //     vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    // ).unwrap(); 
    //
    let img_hist = gray_histogram(&my_img);
    let histogram_img = draw_histogram(1000, 500, &img_hist);
    display_image("histogram", &histogram_img.convert(), 800, 801);
} 
