//! Operations that only deal with one pixel

use image::{GrayAlphaImage, LumaA};

const MAX_VALUE: u8 = 255;

/// inverts image in place
pub fn invert_grayscale_mut(mut image: GrayAlphaImage) -> GrayAlphaImage {
    // for y in 0..image.height() {
    //     for x in 0..image.width() {
    //         let mut pixel = image.get_pixel_mut(x, y);
    //         pixel.data[0 as usize] = MAX_VALUE - pixel.data[0 as usize];
    //         pixel.data[1 as usize] = MAX_VALUE;
    //     }
    // }
    for pixel in image.enumerate_pixels_mut() {
        let inverted_pixel = LumaA([MAX_VALUE - pixel.2.data[0 as usize], MAX_VALUE]);
        pixel.2.data[0 as usize] = MAX_VALUE - pixel.2.data[0 as usize];
        pixel.2.data[1 as usize] = MAX_VALUE;
    }
    image
}

/// inverts copy of input image
pub fn invert_grayscale(image: &GrayAlphaImage) -> GrayAlphaImage {
    let mut inverted_image = image.clone();
    for pixel in image.enumerate_pixels() {
        // println!("{:?}", pixel);
        let inverted_pixel = image::LumaA([MAX_VALUE - pixel.2.data[0 as usize], MAX_VALUE]);
        inverted_image.put_pixel(pixel.0, pixel.1, inverted_pixel);
    }
    inverted_image
}
