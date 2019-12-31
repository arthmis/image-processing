use image::imageops::resize;
use image::FilterType;
use image::RgbaImage;

pub mod blur;
pub mod conversion;
pub mod edge_detection;
pub mod exposure;
pub mod histogram;
pub mod matrix_ops;
pub mod pixel_ops;
#[cfg(feature = "display-window")]
pub mod window;

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
