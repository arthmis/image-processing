use image::{GrayImage, ImageBuffer};
use image::{GenericImage, GenericImageView};
use image::Luma;

use crate::clamp;
use crate::pixel_ops::threshold_mut;

pub fn sobel_mut(image: &mut GrayImage) {

    let mut image_copy = image.clone();
    let mut new_pixel: f32 = 0.0;
    let (width, height) = image.dimensions();
    
    // sobel_y(image);
    // sobel_x(&mut image_copy);

    let mut kernel_image: GrayImage = ImageBuffer::new(width, height);
    sobel_y_inner(image, &mut kernel_image);
    sobel_x_inner(&mut image_copy, &mut kernel_image);

    for (sobel_y, sobel_x) in image.pixels_mut().zip(image_copy.pixels()) {
        new_pixel = (sobel_x[0] as f32).powi(2) + (sobel_y[0] as f32).powi(2);
        sobel_y[0] = new_pixel.sqrt() as u8;
    }
}

fn sobel_x_inner(image: &mut GrayImage, kernel_image: &mut GrayImage) {
    let kernel_one: [i32; 3] = [1, 0, -1];
    let kernel_two = [1, 2, 1];

    let (width, height) = image.dimensions();

    let mut sum: i32 = 0;
    let radius = 1;
    for y in 0..height {
        for x in 0..width {
            let begin = x as i32 - radius;
            let end = x as i32 + radius;
            for (i, kernel_val) in (begin..=end).zip(kernel_one.iter()) {
                if i < 0 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else if i >= width as i32 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else {
                    unsafe {
                        sum += image.unsafe_get_pixel(i as u32, y).0[0] as i32 * kernel_val;
                    }
                }
            }
            kernel_image.get_pixel_mut(x, y).0[0] = sum.abs() as u8;

            sum = 0;
        }
    }

    // vertical kernel
    sum = 0;
    for y in 0..height{
        for x in 0..width {
            let begin = y as i32 - radius;
            let end = y as i32 + radius;
            for (i, kernel_val) in (begin..=end).zip(kernel_two.iter()) {
                if i < 0 {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else if i >= height as i32 {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, i as u32).0[0] as i32 * kernel_val;
                    }
                }
            }
            image.get_pixel_mut(x, y).0[0] = clamp(sum, 0, 255) as u8;

            sum = 0;
        }
    }
}

fn sobel_y_inner(image: &mut GrayImage, kernel_image: &mut GrayImage) {
    let kernel_one: [i32; 3] = [1, 0, -1];
    let kernel_two = [1, 2, 1];

    let (width, height) = image.dimensions();

    let mut sum: i32 = 0;

    let radius = 1;

    // horizontal convolution
    for y in 0..height {
        for x in 0..width {
            let begin = x as i32 - radius;
            let end = x as i32 + radius;
            for (i, kernel_val) in (begin..=end).zip(kernel_two.iter()) {
                if i < 0 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else if i >= width as i32 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else {
                    unsafe {
                        sum += image.unsafe_get_pixel(i as u32, y).0[0] as i32 * kernel_val;
                    }
                }
            }
            kernel_image.get_pixel_mut(x, y).0[0] = clamp(sum, 0, 255) as u8;

            sum = 0;
        }
    }

    sum = 0;
    // vertical convolution 
    for y in 0..height{
        for x in 0..width {
            let begin = y as i32 - radius;
            let end = y as i32 + radius;
            for (i, kernel_val) in (begin..=end).zip(kernel_one.iter()) {
                if i < 0 {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else if i >= height as i32 {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, i as u32).0[0] as i32 * kernel_val;
                    }
                }
            }
            image.get_pixel_mut(x, y).0[0] = sum.abs() as u8;

            sum = 0;
        }
    }
}

pub fn sobel_x(image: &mut GrayImage) {
    let kernel_one: [i32; 3] = [1, 0, -1];
    let kernel_two = [1, 2, 1];

    let (width, height) = image.dimensions();

    let mut kernel_image: GrayImage = ImageBuffer::new(width, height);
    let mut sum: i32 = 0;
    let radius = 1;
    for y in 0..height {
        for x in 0..width {
            let begin = x as i32 - radius;
            let end = x as i32 + radius;
            for (i, kernel_val) in (begin..=end).zip(kernel_one.iter()) {
                if i < 0 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else if i >= width as i32 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else {
                    unsafe {
                        sum += image.unsafe_get_pixel(i as u32, y).0[0] as i32 * kernel_val;
                    }
                }
            }
            kernel_image.get_pixel_mut(x, y).0[0] = sum.abs() as u8;

            sum = 0;
        }
    }

    // vertical kernel
    sum = 0;
    for y in 0..height{
        for x in 0..width {
            let begin = y as i32 - radius;
            let end = y as i32 + radius;
            for (i, kernel_val) in (begin..=end).zip(kernel_two.iter()) {
                if i < 0 {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else if i >= height as i32 {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, i as u32).0[0] as i32 * kernel_val;
                    }
                }
            }
            image.get_pixel_mut(x, y).0[0] = clamp(sum, 0, 255) as u8;

            sum = 0;
        }
    }
}

pub fn sobel_y(image: &mut GrayImage) {
    let kernel_one: [i32; 3] = [1, 0, -1];
    let kernel_two = [1, 2, 1];

    let (width, height) = image.dimensions();
    let mut kernel_image: GrayImage = ImageBuffer::new(width, height);

    let mut sum: i32 = 0;

    let radius = 1;
    // horizontal convolution
    for y in 0..height {
        for x in 0..width {
            let begin = x as i32 - radius;
            let end = x as i32 + radius;
            for (i, kernel_val) in (begin..=end).zip(kernel_two.iter()) {
                if i < 0 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else if i >= width as i32 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else {
                    unsafe {
                        sum += image.unsafe_get_pixel(i as u32, y).0[0] as i32 * kernel_val;
                    }
                }
            }
            kernel_image.get_pixel_mut(x, y).0[0] = clamp(sum, 0, 255) as u8;

            sum = 0;
        }
    }

    sum = 0;
    // vertical convolution 
    for y in 0..height{
        for x in 0..width {
            let begin = y as i32 - radius;
            let end = y as i32 + radius;
            for (i, kernel_val) in (begin..=end).zip(kernel_one.iter()) {
                if i < 0 {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else if i >= height as i32 {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, y).0[0] as i32 * kernel_val;
                    }
                } else {
                    unsafe {
                        sum += kernel_image.unsafe_get_pixel(x, i as u32).0[0] as i32 * kernel_val;
                    }
                }
            }
            image.get_pixel_mut(x, y).0[0] = sum.abs() as u8;

            sum = 0;
        }
    }
}