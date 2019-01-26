//! Operations that only deal with one pixel

use image::{GrayAlphaImage, LumaA};

const MAX_VALUE: u8 = 255;
const MIN_VALUE: u8 = 0;

/// contrast adjustments
pub fn contrast(image: &GrayAlphaImage, contrast: u8) -> GrayAlphaImage {
    let contrast: f32 = {
        let mut normalized_contrast = f32::from(contrast);
        normalized_contrast * 2.0 / 200.0
    };
    let mut new_image = image.clone();
    for pixel in image.enumerate_pixels() {
        let mut new_pixel_value = f32::from(pixel.2.data[0 as usize]) * contrast;
        let new_pixel_value = new_pixel_value.round() as i16;
        let new_pixel_value = if new_pixel_value < 0 {
            new_pixel_value.max(i16::from(MIN_VALUE)) as u8
        } else if new_pixel_value > 255 {
            new_pixel_value.min(i16::from(MAX_VALUE)) as u8
        } else {
            new_pixel_value as u8
        };
        let new_pixel = LumaA([new_pixel_value, MAX_VALUE]);
        new_image.put_pixel(pixel.0, pixel.1, new_pixel);

    }

    new_image
}

/// brightness adjustments
// TODO normalize the brightness input to make it easier to reason about the brightness adjustments
pub fn brightness(image: &GrayAlphaImage, brightness: i16) -> GrayAlphaImage {
    let mut new_image = image.clone();
    for pixel in image.enumerate_pixels() {
        let mut new_pixel_value = i16::from(pixel.2.data[0 as usize]) + brightness;
        if brightness < 0 {
            new_pixel_value.max(i16::from(MIN_VALUE));
        } else {
            new_pixel_value.min(i16::from(MAX_VALUE));
        }
        let new_pixel = LumaA([new_pixel_value as u8, MAX_VALUE]);
        new_image.put_pixel(pixel.0, pixel.1, new_pixel);
    }

    new_image
}

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
        pixel.2.data[0 as usize] = MAX_VALUE - pixel.2.data[0 as usize];
    }
    image
}

/// inverts copy of input image
pub fn invert_grayscale(image: &GrayAlphaImage) -> GrayAlphaImage {
    let mut inverted_image = image.clone();
    for pixel in image.enumerate_pixels() {
        // println!("{:?}", pixel);
        let inverted_pixel = LumaA([MAX_VALUE - pixel.2.data[0 as usize], MAX_VALUE]);
        inverted_image.put_pixel(pixel.0, pixel.1, inverted_pixel);
    }
    inverted_image
}

/// thresholds image in place
pub fn threshold_mut(mut image: GrayAlphaImage, threshold: u8) -> GrayAlphaImage {
    for pixel in image.enumerate_pixels_mut() {
        if pixel.2.data[0 as usize] < threshold {
            pixel.2.data[0 as usize] = MIN_VALUE;
        } else {
            pixel.2.data[0 as usize] = MAX_VALUE;
        }
    }
    image
}

/// thresholds image
pub fn threshold(image: &GrayAlphaImage, threshold: u8) -> GrayAlphaImage {
    let mut threshold_image = image.clone();
    let white = LumaA([MAX_VALUE, MAX_VALUE]);
    let black = LumaA([MIN_VALUE, MAX_VALUE]);
    for pixel in image.enumerate_pixels() {
        if pixel.2.data[0 as usize] < threshold {
            threshold_image.put_pixel(pixel.0, pixel.1, black);
        } else {
            threshold_image.put_pixel(pixel.0, pixel.1, white);
        }
    }
    threshold_image
}
