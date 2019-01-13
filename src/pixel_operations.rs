//! Operations that only deal with one pixel

use image::{GrayAlphaImage};

const MAX_VALUE: u8 = 255;

// pub fn invert_grayscale_mut(image: mut &GrayAlphaImage) -> &GrayAlphaImage {
//     for pixel in image.pixels_mut() {
//
//     }
// }

pub fn invert_grayscale(image: &GrayAlphaImage) -> GrayAlphaImage {
    let inverted_image = image.clone();
    for _pixel in image.enumerate_pixels() {
        
        // inverted_image.put_pixel();
    } 
    inverted_image
}
