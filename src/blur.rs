use image::{GenericImageView, Rgba};
use image::{GrayImage, ImageBuffer, RgbaImage};

use std::f32::consts::{E, PI};
// otherwise known as a box filter

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
                        sum +=
                            horizontal_blur_image.unsafe_get_pixel(x, y).0[0] as f32 * filter_val;
                    }
                } else if i >= height as i32 {
                    unsafe {
                        sum +=
                            horizontal_blur_image.unsafe_get_pixel(x, y).0[0] as f32 * filter_val;
                    }
                } else {
                    unsafe {
                        sum += horizontal_blur_image.unsafe_get_pixel(x, i as u32).0[0] as f32
                            * filter_val;
                    }
                }
            }
            image.get_pixel_mut(x, y).0[0] = sum.round() as u8;
        }
    }
}

// pub fn box_filter_mut_alternate(filter: MeanKernel, image: &mut RgbaImage) {
//     use crate::matrix_ops::transpose;

//     let (width, height) = image.dimensions();

//     // want the truncated value of this division, hence not using float
//     let radius: i32 = filter.size() as i32 / 2;

//     // let mut new_image: RgbaImage = ImageBuffer::new(width, height);
//     let mut new_image: RgbaImage = ImageBuffer::new(width, height);
//     let mut vertical_image: RgbaImage = ImageBuffer::new(height, width);
//     let mut vertical_image_blur: RgbaImage = ImageBuffer::new(height, width);

//     // blur pixels row wise
//     horizontal_blur(radius, image, &mut new_image, width, height);
//     transpose(&new_image, &mut vertical_image);

//     // blur pixels column wise
//     horizontal_blur(
//         radius,
//         &vertical_image,
//         &mut vertical_image_blur,
//         height,
//         width,
//     );
//     transpose(&vertical_image_blur, image);
// }

trait SwapDimension {
    fn swap_dimensions(&mut self);
}

impl SwapDimension for RgbaImage {
    fn swap_dimensions(&mut self) {
        let (width, height) = self.dimensions();
        let temp = core::mem::replace(self, ImageBuffer::new(0, 0));
        let temp: RgbaImage = ImageBuffer::from_raw(height, width, temp.into_raw()).unwrap();
        *self = temp;
    }
}

// got the algorithm for this box blur from here
// https://fgiesen.wordpress.com/2012/07/30/fast-blurs-1/
// http://elynxsdk.free.fr/ext-docs/Blur/Fast_box_blur.pdf
// this filter still isn't complete because it can't take even sized filters
pub fn box_filter_mut(filter: f32, mut image: &mut RgbaImage) {
    // use crate::matrix_ops::transpose_generic;
    use crate::matrix_ops::transpose_rgba;
    use image::Pixel;

    let (width, height) = image.dimensions();

    // want the truncated value of this division, hence not using float
    let radius = filter / 2.0;

    // let white_pixel = [255, 255, 255, 255_u8];
    let mut new_image: RgbaImage =
        ImageBuffer::from_pixel(width, height, Rgba::from_channels(255, 255, 255, 255));
    // let mut new_image: RgbaImage = ImageBuffer::new(width, height);

    // blur pixels row wise
    horizontal_blur(radius, &image, &mut new_image);
    // horizontal_blur_alternate(radius as f32, &image, &mut new_image);

    // swaps dimensions to allow transposing new_image into image
    image.swap_dimensions();
    transpose_rgba(&new_image, &mut image);

    // swaps dimensions to allow image to blur vertically and write to new_image
    new_image.swap_dimensions();

    // blur pixels column wise
    horizontal_blur(radius, &image, &mut new_image);
    // horizontal_blur_alternate(radius as f32, &image, &mut new_image);

    // swaps image back to its original dimensions
    image.swap_dimensions();
    transpose_rgba(&new_image, &mut image);
}

// this assume the kernel is smaller than the image, so it isn't absolutely
// robust for all size kernels and image sizes
fn horizontal_blur(radius: f32, image: &RgbaImage, blur_image: &mut RgbaImage) {
    assert!(image.width() == blur_image.width());
    assert!(image.height() == blur_image.height());

    let (width, height) = image.dimensions();

    let scale = 1.0 / (2.0 * radius + 1.0);
    let (radius, alpha) = {
        let m = radius as i32;
        let alpha = radius - m as f32;
        (m, alpha)
    };

    let (width, height) = (width as i32, height as i32);

    let lerp = |t, a, b| a + t * (b - a);

    for y in 0..height {
        let (mut sum_red, mut sum_green, mut sum_blue) = {
            let mut sum_red = 0.0_f32;
            let mut sum_green = 0.0_f32;
            let mut sum_blue = 0.0_f32;

            let begin = 0 - radius;
            let end = radius;
            for i in begin..=end {
                if i < 0 {
                    let pixel = image[(0, y as u32)];
                    sum_red += pixel[0] as f32;
                    sum_green += pixel[1] as f32;
                    sum_blue += pixel[2] as f32;
                } else if i >= width as i32 {
                    let pixel = image[(width as u32 - 1, y as u32)];
                    sum_red += pixel[0] as f32;
                    sum_green += pixel[1] as f32;
                    sum_blue += pixel[2] as f32;
                } else {
                    let pixel = image[(i as u32, y as u32)];
                    sum_red += pixel[0] as f32;
                    sum_green += pixel[1] as f32;
                    sum_blue += pixel[2] as f32;
                }
            }

            let begin_pixel = image[(0, y as u32)];

            // perform a check to see what size the kernel is in relation
            // to the width of the image
            let end_pixel = image[(end as u32, y as u32)];

            sum_red += alpha * (end_pixel[0] as f32 + begin_pixel[0] as f32);
            sum_green += alpha * (end_pixel[1] as f32 + begin_pixel[1] as f32);
            sum_blue += alpha * (end_pixel[2] as f32 + begin_pixel[2] as f32);

            (sum_red, sum_green, sum_blue)
        };

        for x in 0..width {
            let pixel = blur_image.get_pixel_mut(x as u32, y as u32);
            pixel[0] = (sum_red * scale).round() as u8;
            pixel[1] = (sum_green * scale).round() as u8;
            pixel[2] = (sum_blue * scale).round() as u8;

            let begin_pixel = if x - radius < 0 {
                image[(0, y as u32)]
            } else {
                image[((x - radius) as u32, y as u32)]
            };

            let before_begin_pixel = if x - radius - 1 < 0 {
                image[(0, y as u32)]
            } else {
                image[((x - radius - 1) as u32, y as u32)]
            };

            let end_pixel = if x + radius + 1 >= width {
                image[(width as u32 - 1, y as u32)]
            } else {
                image[((x + radius + 1) as u32, y as u32)]
            };

            let after_end_pixel = if x + radius + 2 >= width {
                image[(width as u32 - 1, y as u32)]
            } else {
                image[((x + radius + 2) as u32, y as u32)]
            };

            sum_red += lerp(alpha, end_pixel[0] as f32, after_end_pixel[0] as f32);
            sum_red -= lerp(alpha, begin_pixel[0] as f32, before_begin_pixel[0] as f32);

            sum_green += lerp(alpha, end_pixel[1] as f32, after_end_pixel[1] as f32);
            sum_green -= lerp(alpha, begin_pixel[1] as f32, before_begin_pixel[1] as f32);

            sum_blue += lerp(alpha, end_pixel[2] as f32, after_end_pixel[2] as f32);
            sum_blue -= lerp(alpha, begin_pixel[2] as f32, before_begin_pixel[2] as f32);
        }
    }
}

pub fn naive_box_filter_mut(filter: u32, image: &mut RgbaImage) {
    let (width, height) = image.dimensions();

    // want the truncated value of this division, hence not using float
    let filter_radius: i32 = filter as i32 / 2;

    let mut horizontal_blur_image: RgbaImage = ImageBuffer::new(width, height);

    // blur horizontally
    for y in 0..height {
        for x in 0..width {
            let mut sum_red: f32 = 0.0;
            let mut sum_green: f32 = 0.0;
            let mut sum_blue: f32 = 0.0;
            let begin: i32 = x as i32 - filter_radius;
            let end: i32 = x as i32 + filter_radius;
            // virtually loops through filter and image
            // doesn't actually access the filter because everything is 1
            for i in begin..=end {
                if i < 0 {
                    sum_red += image.get_pixel(x, y).0[0] as f32;
                    sum_green += image.get_pixel(x, y).0[1] as f32;
                    sum_blue += image.get_pixel(x, y).0[2] as f32;
                } else if i >= width as i32 {
                    sum_red += image.get_pixel(x, y).0[0] as f32;
                    sum_green += image.get_pixel(x, y).0[1] as f32;
                    sum_blue += image.get_pixel(x, y).0[2] as f32;
                } else {
                    sum_red += image.get_pixel(i as u32, y).0[0] as f32;
                    sum_green += image.get_pixel(i as u32, y).0[1] as f32;
                    sum_blue += image.get_pixel(i as u32, y).0[2] as f32;
                }
            }
            horizontal_blur_image.get_pixel_mut(x, y).0[0] =
                (sum_red as f32 / filter as f32).round() as u8;
            horizontal_blur_image.get_pixel_mut(x, y).0[1] =
                (sum_green as f32 / filter as f32).round() as u8;
            horizontal_blur_image.get_pixel_mut(x, y).0[2] =
                (sum_blue as f32 / filter as f32).round() as u8;
        }
    }

    // blur vertically
    for y in 0..height {
        for x in 0..width {
            let mut sum_red: f32 = 0.0;
            let mut sum_green: f32 = 0.0;
            let mut sum_blue: f32 = 0.0;
            let begin: i32 = y as i32 - filter_radius;
            let end: i32 = y as i32 + filter_radius;
            for i in begin..=end {
                if i < 0 {
                    sum_red += horizontal_blur_image.get_pixel(x, y).0[0] as f32;
                    sum_green += horizontal_blur_image.get_pixel(x, y).0[1] as f32;
                    sum_blue += horizontal_blur_image.get_pixel(x, y).0[2] as f32;
                } else if i >= height as i32 {
                    sum_red += horizontal_blur_image.get_pixel(x, y).0[0] as f32;
                    sum_green += horizontal_blur_image.get_pixel(x, y).0[1] as f32;
                    sum_blue += horizontal_blur_image.get_pixel(x, y).0[2] as f32;
                } else {
                    sum_red += horizontal_blur_image.get_pixel(x, i as u32).0[0] as f32;
                    sum_green += horizontal_blur_image.get_pixel(x, i as u32).0[1] as f32;
                    sum_blue += horizontal_blur_image.get_pixel(x, i as u32).0[2] as f32;
                }
            }
            image.get_pixel_mut(x, y).0[0] = (sum_red as f32 / filter as f32).round() as u8;
            image.get_pixel_mut(x, y).0[1] = (sum_green as f32 / filter as f32).round() as u8;
            image.get_pixel_mut(x, y).0[2] = (sum_blue as f32 / filter as f32).round() as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fast_box_blur() {
        let mut image = image::open("./images/england-hampton-court-palace.jpg")
            .expect("image not found")
            .to_rgba();
        let mut fast_image = image.clone();

        let size = 3;
        box_filter_mut(MeanKernel::new(size), &mut fast_image);

        for (naive, fast) in image.pixels().zip(fast_image.pixels()) {
            assert_eq!(naive[0], fast[0]);
            assert_eq!(naive[1], fast[1]);
            assert_eq!(naive[2], fast[2]);
        }
    }
}
