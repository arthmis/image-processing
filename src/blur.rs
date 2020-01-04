// use image::RgbaImage;
use image::{RgbaImage, GrayImage, ImageBuffer};
use image::{GenericImageView};

use std::f32::consts::{E, PI};
// otherwise known as a box filter
#[derive(Clone, Copy)]
pub struct MeanKernel(u32);


const STRIDE: usize = 4;

impl MeanKernel {
    pub fn new(size: u32) -> Self {
        assert!(size % 2 != 0, "Size needs to be odd. Size was: {}", size);
        MeanKernel(size) 
    }

    pub fn size(&self) -> u32 {
        self.0
    }
}

#[derive(Clone)]
pub struct GaussianKernel {
    pub sigma: f32,
    // pub size: u32,
    pub one_dimension_filter: Vec<f32>,
}

impl GaussianKernel {
    // think about making sigma be floating point and 1.0 and higher
    pub fn new(sigma: u32) -> Self {
        assert!(sigma >= 1, "Sigma has to be 1 or greater: {}", sigma);

        // using a standard deviation of 2 instead of 3 because
        // 95% coverage is good enough while 3 std would cover 99.7%
        let standard_deviation = 2;

        // the `+ 1` makes the size odd 
        let size = standard_deviation * sigma * 2 + 1;

        // figure out how to round the size up to odd when sigma can be f32
        assert!(size % 2 != 0, "Size needs to be odd. Size was: {}", size);

        let begin = size as i32 / 2 * -1;
        let end = size as i32 / 2;

        let mut filter = vec![0_f32; size as usize];

        for (i, x) in filter.iter_mut().zip(begin..=end) {
            *i = x as f32;
        }

        let sigma = sigma as f32;
        // create gauss filter values
        for x in filter.iter_mut() {
            let exponent_of_e = x.powi(2) / sigma.powi(2) / -2.0;
            let e = E.powf(exponent_of_e); 
            *x = 1.0 / ((2.0 * PI).sqrt() * sigma) * e;
        }

        GaussianKernel {
            sigma: sigma as f32,
            one_dimension_filter: filter,
        }
    }
}

pub fn gaussian_filter_mut(filter: &GaussianKernel, image: &mut GrayImage) {
    let filter = filter.one_dimension_filter.as_slice();

    let (width, height) = image.dimensions();

    let mut horizontal_blur_image: GrayImage = ImageBuffer::new(width, height);

    // want the truncated value of this division, hence not using float
    let filter_radius: i32 = filter.len() as i32 / 2;

    // blur horizontally
    for y in 0..height {
        for x in 0..width {
            let mut sum: f32 = 0.0;
            let begin: i32 = x as i32 - filter_radius;
            let end: i32 = x as i32 + filter_radius;  
            for (i, filter_val) in (begin..=end).zip(filter.iter()) {
                if i < 0 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as f32 * filter_val; 
                    }
                } else if i >= width as i32 {
                    unsafe {
                        sum += image.unsafe_get_pixel(x, y).0[0] as f32 * filter_val; 
                    }
                } else {
                    unsafe {
                        sum += image.unsafe_get_pixel(i as u32, y).0[0] as f32 * filter_val;
                    }
                }
            }
            horizontal_blur_image.get_pixel_mut(x, y).0[0] = sum.round() as u8; 
        }
    }

    // blur vertically
    for y in 0..height {
        for x in 0..width {
            let mut sum: f32 = 0.0;
            let begin: i32 = y as i32 - filter_radius;
            let end: i32 = y as i32 + filter_radius;  
           
            for (i, filter_val) in (begin..=end).zip(filter.iter()) {
                if i < 0 {
                    unsafe {
                        sum += horizontal_blur_image.unsafe_get_pixel(x, y).0[0] as f32 * filter_val; 
                    }
                } else if i >= height as i32 {
                    unsafe {
                        sum += horizontal_blur_image.unsafe_get_pixel(x, y).0[0] as f32 * filter_val; 
                    }
                } else {
                    unsafe {
                        sum += horizontal_blur_image.unsafe_get_pixel(x, i as u32).0[0] as f32 * filter_val;
                    }
                }
            }
            image.get_pixel_mut(x, y).0[0] = sum.round() as u8; 
        }
    }
}

// got the algorithm for this box blur from here
// https://fgiesen.wordpress.com/2012/07/30/fast-blurs-1/
// http://elynxsdk.free.fr/ext-docs/Blur/Fast_box_blur.pdf
// this filter still isn't complete because it can't take even sized filters
// pub fn box_filter_mut(filter: MeanKernel, image: &mut GrayImage) {
//     use crate::matrix_ops::*;

//     let (width, height) = image.dimensions();

//     // want the truncated value of this division, hence not using float
//     let radius: i32 = filter.size() as i32 / 2;

//     let mut new_image: GrayImage = ImageBuffer::new(width, height);

//     horizontal_blur(radius, image, &mut new_image, width, height);
//     transpose(&new_image, &mut *image, width as usize, height as usize);
//     horizontal_blur(radius, image, &mut new_image, height, width);
//     transpose(&new_image, &mut *image, height as usize, width as usize);
// } 

// fn horizontal_blur(radius: i32, image: &[u8], blur_image: &mut [u8], width: u32, height: u32) {
//     let scale = 1.0 / (2.0 * radius as f32 + 1.0);

//     let (width, height) = (width as i32, height as i32);
//     for y in 0..height {
//         let mut sum = {
//             let mut sum: f32 = 0.0;
//             let begin = 0 - radius;
//             let end = 0 + radius;
//             for i in begin..=end {
//                 if i < 0 {
//                     unsafe {
//                         sum += *image.get_unchecked((y * width) as usize) as f32;
//                     }
//                 } else if i >= width as i32 {
//                     unsafe {
//                         sum += *image.get_unchecked((y * width + width-1) as usize) as f32;

//                     }
//                 } else {
//                     unsafe {
//                         sum += *image.get_unchecked((y * width + i) as usize) as f32;
//                     }
//                 }
//             } 
//             sum
//         };
//         let mut end_pixel = 0.0_f32;
//         let mut begin_pixel = 0.0_f32;
//         unsafe {
//             end_pixel = *image.get_unchecked((y * width + width - 1) as usize) as f32; 
//             begin_pixel = *image.get_unchecked((y * width) as usize) as f32;
//         }
//         for x in 0..width {
//             let x = x as i32;

//             unsafe {
//                 let elem = blur_image.get_unchecked_mut((y * width + x) as usize);
//                 *elem = (sum * scale).round() as u8;
//             }

//             if x + radius + 1 >= width as i32 && x - radius < 0 {
//                 unsafe {
//                     sum += end_pixel - begin_pixel;
//                 }
//             } else if x + radius + 1 >= width as i32 {
//                 unsafe {
//                     sum += end_pixel - *image.get_unchecked((y * width + x - radius) as usize) as f32;
//                 }
//             } else if x - radius < 0 {
//                 unsafe {
//                     sum += *image.get_unchecked((y * width + x + radius + 1) as usize) as f32 - begin_pixel;
//                 }
//             } else {
//                 unsafe {
//                     sum += *image.get_unchecked((y * width + x + radius + 1) as usize) as f32 - *image.get_unchecked((y * width + x - radius) as usize) as f32;

//                 }
//             }
//         }
//     } 
// }

pub fn box_filter_mut_alternate(filter: MeanKernel, image: &mut RgbaImage) {
    use crate::matrix_ops::transpose;

    let (width, height) = image.dimensions();

    // want the truncated value of this division, hence not using float
    let radius: i32 = filter.size() as i32 / 2;

    // let mut new_image: RgbaImage = ImageBuffer::new(width, height);
    let mut new_image: RgbaImage = ImageBuffer::new(width, height);
    let mut vertical_image: RgbaImage = ImageBuffer::new(height, width);
    let mut vertical_image_blur: RgbaImage = ImageBuffer::new(height, width);

    // blur pixels row wise
    horizontal_blur(radius, image, &mut new_image, width, height);
    transpose(&new_image, &mut vertical_image);

    // blur pixels column wise
    horizontal_blur(radius, &vertical_image, &mut vertical_image_blur, height, width);
    transpose(&vertical_image_blur, image);
} 
pub fn box_filter_mut(filter: MeanKernel, image: &mut RgbaImage) {
    use crate::matrix_ops::transpose_generic;

    let (width, height) = image.dimensions();

    // want the truncated value of this division, hence not using float
    let radius: i32 = filter.size() as i32 / 2;

    let mut new_image: RgbaImage = ImageBuffer::new(width, height);

    // blur pixels row wise
    horizontal_blur(radius, image, &mut new_image, width, height);
    transpose_generic(&new_image, image, width as usize, height as usize, STRIDE);

    // blur pixels column wise
    horizontal_blur(radius, &image, &mut new_image, height, width);
    transpose_generic(&new_image, image, height as usize, width as usize, STRIDE);

}
const CHANNEL_COUNT: i32 = 4;
fn horizontal_blur(radius: i32, image: &[u8], blur_image: &mut [u8], width: u32, height: u32) {
    let scale = 1.0 / (2.0 * radius as f32 + 1.0);

    let (width, height) = (width as i32, height as i32);
    for y in 0..height {
        let (mut sum_red, mut sum_green, mut sum_blue) = {
            // let mut sum: f32 = 0.0;
            let mut sum_red = 0.0_f32;
            let mut sum_green = 0.0_f32;
            let mut sum_blue = 0.0_f32;

            let begin = 0 - radius;
            let end = 0 + radius;
            for i in begin..=end {
                if i < 0 {
                    // unsafe {
                    //     // sum += *image.get_unchecked((y * width) as usize) as f32;
                    // }
                    // sum += *image.get_unchecked((y * width) as usize) as f32;
                    let pixel_index = (y * width * CHANNEL_COUNT) as usize;

                    // dbg!(pixel_index);

                    sum_red += *image.get(pixel_index).unwrap() as f32;

                    sum_green += *image.get(pixel_index + 1).unwrap() as f32;

                    sum_blue += *image.get(pixel_index + 2).unwrap() as f32;

                } else if i >= width as i32 {
                    // unsafe {
                    //     // sum += *image.get_unchecked((y * width + width-1) as usize) as f32;

                    // }
                    // sum += *image.get_unchecked((y * width + width-1) as usize) as f32;
                    let pixel_index = (y * width * CHANNEL_COUNT + (width - 1) * CHANNEL_COUNT) as usize;
                    // dbg!(pixel_index);
                    sum_red += *image.get(pixel_index).unwrap() as f32;
                    sum_green += *image.get(pixel_index + 1).unwrap() as f32;
                    sum_blue += *image.get(pixel_index + 2).unwrap() as f32;

                } else {
                    // unsafe {
                    //     // sum += *image.get_unchecked((y * width + i) as usize) as f32;
                    //     let pixel = (y * (width * CHANNEL_COUNT) + i * CHANNEL_COUNT) as usize;
                    //     sum_red += *image.get_unchecked(pixel) as f32;
                    //     sum_green += *image.get_unchecked(pixel + 1) as f32;
                    //     sum_blue += *image.get_unchecked(pixel + 2) as f32;
                    // }
                        // sum += *image.get_unchecked((y * width + i) as usize) as f32;
                    let pixel_index = (y * width * CHANNEL_COUNT + i * CHANNEL_COUNT) as usize;
                    // dbg!(pixel_index);
                    sum_red += *image.get(pixel_index).unwrap() as f32;
                    sum_green += *image.get(pixel_index + 1).unwrap() as f32;
                    sum_blue += *image.get(pixel_index + 2).unwrap() as f32;
                }
            } 
            (sum_red, sum_green, sum_blue)
        };
        // let mut end_pixel = 0.0_f32;
        // let mut begin_pixel = 0.0_f32;
        // unsafe {
        //     end_pixel = *image.get_unchecked((y * width + width - 1) as usize) as f32; 
        //     begin_pixel = *image.get_unchecked((y * width) as usize) as f32;
        // }

        let begin_index = (y * width * CHANNEL_COUNT) as usize;
        let begin_pixel_red = *image.get(begin_index).unwrap() as f32;
        let begin_pixel_green = *image.get(begin_index + 1).unwrap() as f32;
        let begin_pixel_blue = *image.get(begin_index + 2).unwrap() as f32;
        // dbg!(begin_index);

        let end_index = (y * width * CHANNEL_COUNT + (width - 1) * CHANNEL_COUNT) as usize;
        // dbg!(end_index);
        let end_pixel_red = *image.get(end_index).unwrap() as f32;
        let end_pixel_green = *image.get(end_index + 1).unwrap() as f32;
        let end_pixel_blue = *image.get(end_index + 2).unwrap() as f32;

        for x in 0..width {

            // unsafe {
            //     let luma = blur_image.get_unchecked_mut((y * width + x) as usize);
            //     *luma = (sum * scale).round() as u8;
            // }
            let current_pixel = (y * width * CHANNEL_COUNT + x * CHANNEL_COUNT) as usize;
            // dbg!(current_pixel);
            let red = blur_image.get_mut(current_pixel).unwrap();
            *red = (sum_red * scale).round() as u8;
            // dbg!(*red);

            let green = blur_image.get_mut(current_pixel + 1).unwrap();
            *green = (sum_green * scale).round() as u8;
            // dbg!(*green);

            let blue = blur_image.get_mut(current_pixel + 2).unwrap();
            *blue = (sum_blue * scale).round() as u8;
            // dbg!(*blue);
            let alpha = blur_image.get_mut(current_pixel + 3).unwrap();
            *alpha = 255;
            if x + radius + 1 >= width as i32 && x - radius < 0 {
                // unsafe {
                //     sum += end_pixel - begin_pixel;
                // }
                sum_red += end_pixel_red - begin_pixel_red;

                sum_green += end_pixel_green - begin_pixel_green;

                sum_blue += end_pixel_blue - begin_pixel_blue;

            } else if x + radius + 1 >= width as i32 {
                // unsafe {
                //     sum += end_pixel - *image.get_unchecked((y * width + x - radius) as usize) as f32;
                // }
                let pixel_index = (y * width * CHANNEL_COUNT + (x - radius) * CHANNEL_COUNT) as usize;
                sum_red += end_pixel_red - *image.get(pixel_index).unwrap() as f32;
                // dbg!(end_pixel_red - *image.get(pixel_index).unwrap() as f32);
                sum_green += end_pixel_green - *image.get(pixel_index + 1).unwrap() as f32;
                // dbg!(end_pixel_green - *image.get(pixel_index + 1).unwrap() as f32);
                sum_blue += end_pixel_blue - *image.get(pixel_index + 2).unwrap() as f32;
                // dbg!(end_pixel_blue - *image.get(pixel_index + 2).unwrap() as f32);
                // println!();
            } else if x - radius < 0 {
                // unsafe {
                //     sum += *image.get_unchecked((y * width + x + radius + 1) as usize) as f32 - begin_pixel;
                // }
                // sum += *image.get((y * width + x + radius + 1) as usize) as f32 - begin_pixel;
                let pixel_index = (y * width * CHANNEL_COUNT + (x + radius + 1) * CHANNEL_COUNT) as usize;
                // dbg!(pixel_index);
                sum_red += *image.get(pixel_index).unwrap() as f32 - begin_pixel_red;
                sum_green += *image.get(pixel_index + 1).unwrap() as f32 - begin_pixel_green;
                sum_blue += *image.get(pixel_index + 2).unwrap() as f32 - begin_pixel_blue;
            } else {
                // unsafe {
                //     sum += *image.get_unchecked((y * width + x + radius + 1) as usize) as f32 - *image.get_unchecked((y * width + x - radius) as usize) as f32;
                // }
                // sum += *image.get((y * width + x + radius + 1) as usize) as f32 - *image.get((y * width + x - radius) as usize) as f32;
                let last_pixel_index = (y * width * CHANNEL_COUNT + (x + radius + 1) * CHANNEL_COUNT) as usize;
                let first_pixel_index = (y * width * CHANNEL_COUNT + (x - radius) * CHANNEL_COUNT) as usize;
                sum_red += *image.get(last_pixel_index).unwrap() as f32 - *image.get(first_pixel_index).unwrap() as f32;
                sum_green += *image.get(last_pixel_index + 1).unwrap() as f32 - *image.get(first_pixel_index + 1).unwrap() as f32;
                sum_blue += *image.get(last_pixel_index + 2).unwrap() as f32 - *image.get(first_pixel_index + 2).unwrap() as f32;
            }
        }
    } 
}