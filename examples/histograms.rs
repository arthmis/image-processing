extern crate image;
extern crate image_processing;

use image::{
    GrayImage, 
};

use image_processing::statistics::histogram::*;

fn main() {
    let img = image::open("images/london-bridge.jpg")
       .expect("Image not found").to_luma();

    // let some = integral_image(&img); 

    // let my_img = GrayImage::from_raw(
    //     4, 
    //     3, 
    //     vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]
    // ).unwrap(); 
    //
    let img_hist = gray_histogram(&img);
    draw_histogram(256, 256, "test2.png", &img_hist);
    draw_histogram(512, 256, "test.png", &img_hist);

} 
