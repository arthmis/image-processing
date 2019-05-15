extern crate image;
extern crate image_processing;

use image::{GenericImage, GrayAlphaImage, GrayImage, LumaA, Pixel};

use image_processing::statistics::integral_image::*;

fn main() {
    let _img = image::open("images/london-bridge.jpg")
        .expect("Image not found")
        .to_luma();

    // let some = integral_image(&img);

    let my_img = GrayImage::from_raw(4, 3, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]).unwrap();
    let (some, thing) = integral_image(&my_img);
    println!();
    for y in 0..my_img.height() {
        for x in 0..my_img.width() {
            print!("{} ", my_img.get_pixel(x, y)[0]);
        }
        println!();
    }
    println!();
    for y in 0..=my_img.height() {
        for x in 0..=my_img.width() {
            print!("{} ", some.get_point(x, y));
        }
        println!();
    }
    println!();
    for y in 0..=my_img.height() {
        for x in 0..=my_img.width() {
            print!("{} ", thing.get_point(x, y));
        }
        println!();
    }
}
