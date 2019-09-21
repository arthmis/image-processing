use image::{RgbaImage};
use image::imageops::resize;
use image::FilterType;

pub mod histogram;
pub mod pixel_ops;
#[cfg(feature = "display-window")]
pub mod window;
pub mod blur;
pub mod edge_detection;

use pixel_ops::*;
// use histogram::*;

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