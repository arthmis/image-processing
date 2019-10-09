use image::{RgbaImage};
use image::imageops::resize;
use image::FilterType;

pub mod histogram;
pub mod pixel_ops;
#[cfg(feature = "display-window")]
pub mod window;
pub mod blur;
pub mod edge_detection;
pub mod matrix_ops;
pub mod exposure; 

use image::GrayImage;
use image::Primitive;

pub fn clamp<T: Primitive + PartialOrd>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn image_max(image: &GrayImage) -> u8 {

    let mut max = 0;
    for pixel in image.pixels() {
        max = max.max(pixel[0]);
    }
    max
}