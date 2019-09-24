// use image::RgbaImage;
use image::GrayImage;
use image::ImageBuffer;

// otherwise known as a box filter
#[derive(Clone)]
pub struct MeanKernel {
    pub size: u32,
    // can strip this down to only one filter because horizontal
    // and vertical are the same array or i can take them both out
    // because they are never actually used for computations because
    // the values are only 1 which is the identity when multiplied
    pub horizontal: Vec<u8>,
    pub vertical: Vec<u8>,
}

impl MeanKernel {
    pub fn new(size: u32) -> Self {
        assert!(size % 2 != 0, "Size needs to be odd. Size was: {}", size);
        MeanKernel {
            size,
            horizontal: vec![1; size as usize],
            vertical: vec![1; size as usize],
        }
    }
}

#[derive(Clone)]
pub struct GaussianKernel {
    pub sigma: f32,
    // pub size: u32,
    pub one_dimension_filter: Vec<f32>,
}

use std::f32::consts::{E, PI};
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

pub fn gaussian_filter_mut(filter: GaussianKernel, image: &mut GrayImage) {
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
pub fn mean_filter_mut(filter: MeanKernel, image: &mut GrayImage) {
    let horizontal = filter.horizontal.as_slice();

    let (width, height) = image.dimensions();

    let mut horizontal_blur_image: GrayImage = ImageBuffer::new(width, height);
    // want the truncated value of this division, hence not using float
    let filter_radius: i32 = horizontal.len() as i32 / 2;
    let filter_size = horizontal.len();

    // blur horizontally
    for y in 0..height {
        for x in 0..width {
            let mut sum: u32 = 0;
            let begin: i32 = x as i32 - filter_radius;
            let end: i32 = x as i32 + filter_radius;  
            // virtually loops through filter and image
            // doesn't actually access the filter because everything is 1
            for i in begin..=end {
                if i < 0 {
                    sum += image.get_pixel(x, y).0[0] as u32; 
                } else if i >= width as i32 {
                    sum += image.get_pixel(x, y).0[0] as u32; 
                } else {
                    sum += image.get_pixel(i as u32, y).0[0] as u32;
                }
            }
            horizontal_blur_image.get_pixel_mut(x, y).0[0] = (sum as f32 / filter_size as f32) as u8; 
        }
    }

    // blur vertically
    for y in 0..height {
        for x in 0..width {
            let mut sum: u32 = 0;
            let begin: i32 = y as i32 - filter_radius;
            let end: i32 = y as i32 + filter_radius;  
            for i in begin..=end {
                if i < 0 {
                    sum += horizontal_blur_image.get_pixel(x, y).0[0] as u32; 
                } else if i >= height as i32 {
                    sum += horizontal_blur_image.get_pixel(x, y).0[0] as u32; 
                } else {
                    sum += horizontal_blur_image.get_pixel(x, i as u32).0[0] as u32;
                }
            }
            image.get_pixel_mut(x, y).0[0] = (sum as f32 / filter_size as f32) as u8; 
        }
    }

}

pub fn fast_box_flur(filter: MeanKernel, image: &mut GrayImage) {
    use crate::matrix_ops::transpose;

    let (width, height) = image.dimensions();

    // want the truncated value of this division, hence not using float
    let radius: i32 = filter.size() as i32 / 2;
    let size = filter.size();

    let scale = 1.0 / (2.0 * radius as f32 + 1.0);

    let mut transpose_image: GrayImage = ImageBuffer::new(height, width);
    // let mut new_image = ImageBuffer::new(width, height);
    let mut new_image = image.clone();
    let mut transpose_img_scratch: GrayImage = ImageBuffer::new(height, width);

    horizontal_blur(radius, &new_image, &mut *image);
    horizontal_blur(radius, image, &mut new_image);
    transpose(&new_image, &mut transpose_image, width as usize, height as usize);
    horizontal_blur(radius, &transpose_image, &mut transpose_img_scratch);
    transpose(&transpose_img_scratch, &mut *image, height as usize, width as usize);
}

fn horizontal_blur(radius: i32, image: &GrayImage, blur_image: &mut GrayImage) {
    let scale = 1.0 / (2.0 * radius as f32 + 1.0);
    let (width, height) = image.dimensions();

    for y in 0..height {
        let mut sum = {
            let mut sum: f32 = 0.0;
            let begin = 0 - radius;
            let end = 0 + radius;
            for i in begin..=end {
                if i < 0 {
                    sum += image.get_pixel(0, y).0[0] as f32;
                } else if i >= width as i32 {
                    sum += image.get_pixel(width - 1, y).0[0] as f32;
                } else {
                    sum += image.get_pixel(i as u32, y).0[0] as f32;
                }
            } 
            sum
        };
        for x in 0..width {
            let x = x as i32;
            blur_image.get_pixel_mut(x as u32, y).0[0] = (sum * scale).round() as u8;
            if x + radius + 1 >= width as i32 && x - radius < 0 {
                sum += image.get_pixel((width as i32 - 1) as u32, y).0[0] as f32 - image.get_pixel(0, y).0[0] as f32;
            } else if x + radius + 1 >= width as i32 {
                sum += image.get_pixel((width - 1) as u32, y).0[0] as f32 - image.get_pixel((x - radius) as u32, y).0[0] as f32;
            } else if x - radius < 0 {
                sum += image.get_pixel((x + radius + 1) as u32, y).0[0] as f32 - image.get_pixel(0, y).0[0] as f32;
            } else {
                sum += image.get_pixel((x + radius + 1) as u32, y).0[0] as f32 - image.get_pixel((x - radius) as u32, y).0[0] as f32;
            }
        }
    } 
