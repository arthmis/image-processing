//! Operations that only deal with one pixel

use image::{GrayAlphaImage, LumaA};

const MAX_VALUE: u8 = 255;
const MIN_VALUE: u8 = 0;

// TODO add contrast mut function

/// contrast adjustments
/// values from 0-99 decrease contrast and 100-200 increase contrast
pub fn contrast(image: &GrayAlphaImage, contrast: u8) -> GrayAlphaImage {
    let contrast: f32 = {
        let mut normalized_contrast = if contrast > 199 {
            200.0
        } else {
            f32::from(contrast)
        };
        normalized_contrast * 2.0 / 200.0
    };

    let mut new_image = image.clone();
    for pixel in new_image.pixels_mut() {
        let mut new_pixel_value = (f32::from(pixel.data[0 as usize]) - 128.0) * contrast + 128.0;
        let new_pixel_value = new_pixel_value.round() as i64;
        let new_pixel_value = clamp_pixel(new_pixel_value);
        pixel.data[0 as usize] = new_pixel_value;
    }

    new_image
}

/// automatic contrast adjustment
// TODO continue tweaking this to work better and use modified auto contrast
// also change function signature to accept lower and upperbound on range of values
pub fn auto_contrast(image: &GrayAlphaImage) -> GrayAlphaImage {
    let mut new_image = image.clone();

    let min_pixel = {
        let mut min: u8 = 255;
        for pixel in new_image.pixels() {
            if pixel.data[0 as usize] < min {
                min = pixel.data[0 as usize];
            }
        }
        f32::from(min)
    };

    let max_pixel = {
        let mut max: u8 = 0;
        for pixel in new_image.pixels() {
            if pixel.data[0 as usize] > max {
                max = pixel.data[0 as usize];
            }
        }
        f32::from(max)
    };

    for pixel in new_image.pixels_mut() {
        // dbg!(pixel);
        let mut new_pixel_value = f32::from(MIN_VALUE)
            + (f32::from(pixel.data[0 as usize]) - min_pixel) * f32::from(MIN_VALUE)
                / (max_pixel - min_pixel);
        let new_pixel_value = new_pixel_value.round() as i64;
        let new_pixel_value = clamp_pixel(new_pixel_value);
        pixel.data[0 as usize] = new_pixel_value;
    }
    new_image
}

// TODO add brightness mut function

/// brightness adjustments
/// brightness range is [-256, 255] inclusive
/// negative values decrease brightness
pub fn brightness(image: &GrayAlphaImage, brightness: i16) -> GrayAlphaImage {
    let brightness = if brightness > i16::from(MAX_VALUE) {
        255_i64
    } else if brightness < -256 {
        -256_i64
    } else {
        i64::from(brightness)
    };

    let mut new_image = image.clone();

    for pixel in image.enumerate_pixels() {
        let mut new_pixel_value = i64::from(pixel.2.data[0 as usize]) + brightness;
        let new_pixel_value: u8 = clamp_pixel(new_pixel_value);
        let new_pixel = LumaA([new_pixel_value as u8, MAX_VALUE]);
        new_image.put_pixel(pixel.0, pixel.1, new_pixel);
    }

    new_image
}

/// inverts image in place
pub fn invert_grayscale_mut(image: &mut GrayAlphaImage) {
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
}

/// inverts copy of input image
pub fn invert_grayscale(image: &GrayAlphaImage) -> GrayAlphaImage {
    let mut inverted_image = image.clone();
    invert_grayscale_mut(&mut inverted_image);
    inverted_image
}

/// thresholds image in place
pub fn threshold_mut(image: &mut GrayAlphaImage, threshold: u8) {
    for pixel in image.enumerate_pixels_mut() {
        if pixel.2.data[0 as usize] < threshold {
            pixel.2.data[0 as usize] = MIN_VALUE;
        } else {
            pixel.2.data[0 as usize] = MAX_VALUE;
        }
    }
}

/// thresholds image
pub fn threshold(image: &GrayAlphaImage, threshold: u8) -> GrayAlphaImage {
    let mut threshold_image = image.clone();
    threshold_mut(&mut threshold_image, threshold);
    threshold_image
}

/// approximately equalize histogram
pub fn equalize_histogram(image: &GrayAlphaImage) -> GrayAlphaImage {
    use crate::statistics::histogram::cumulative_gray_histogram;

    let mut new_image = image.clone();
    let cumulative_hist = cumulative_gray_histogram(image);
    // let hist = graya_histogram(image);
    let image_width: f64 = f64::from(new_image.width());
    let image_height: f64 = f64::from(new_image.height());

    let max_value: f64 = f64::from(MAX_VALUE);

    for pixel in new_image.pixels_mut() {
        let cumulative_hist_value = f64::from(cumulative_hist.values[pixel.data[0] as usize]);
        let pixel_count = image_width * image_height;
        let new_pixel_value = (cumulative_hist_value / pixel_count * max_value).floor();
        let new_pixel_value = clamp_pixel(new_pixel_value as i64);
        pixel.data[0] = new_pixel_value;
    }

    new_image
}

// TODO make this more efficient by calculating cumulative distribution function when
// calculating the histogram via the function found on pg. 67
// also correct because it currently doesn't work correctly
// and add documentation for it
pub fn match_piecewise_linear_histogram(image: &GrayAlphaImage, reference_image: &GrayAlphaImage) -> GrayAlphaImage {
    use crate::statistics::histogram::cumulative_gray_histogram;
    use crate::statistics::histogram::graya_histogram;

    let mut new_image = image.clone();

    // calculate cumulative distribution functions for both images
    let image_cumulative_histogram: [u32; 256] = cumulative_gray_histogram(&new_image).values;
    let ref_image_cumulative_histogram: [u32; 256] =
        cumulative_gray_histogram(&reference_image).values;
    let ref_image_histogram: [u32; 256] = graya_histogram(&reference_image).values;

    let image_total_pixels = f64::from(new_image.width() * new_image.height());
    let ref_image_total_pixels = f64::from(reference_image.width() * reference_image.height());

    debug_assert!(
        image_total_pixels as u32 == image_cumulative_histogram[255],
        "{} {}",
        image_total_pixels,
        image_cumulative_histogram[255]
    );
    debug_assert!(
        ref_image_total_pixels as u32 == ref_image_cumulative_histogram[255],
        "{} {}",
        ref_image_total_pixels,
        ref_image_cumulative_histogram[255]
    );

    let mut image_cumulative_distribution_function: [f64; 256] = [0.0; 256];
    let mut ref_image_cumulative_distribution_function: [f64; 256] = [0.0; 256];

    for (dist_val, hist_val) in image_cumulative_distribution_function
        .iter_mut()
        .zip(image_cumulative_histogram.iter())
    {
        *dist_val = f64::from(*hist_val) / image_total_pixels;
    }

    for (dist_val, hist_val) in ref_image_cumulative_distribution_function
        .iter_mut()
        .zip(ref_image_cumulative_histogram.iter())
    {
        *dist_val = f64::from(*hist_val) / ref_image_total_pixels;
    }

    for ((i, cdf), cumulative) in ref_image_cumulative_distribution_function.iter().enumerate().zip(ref_image_cumulative_histogram.iter()) {
        println!("index: {}, cdf: {}, cumulative: {} hist: {}", i, cdf, cumulative, ref_image_histogram[i]);
    }

    // create piecewise linear distribution for reference image
    let ref_begin_value: (u8, f64) = (0, ref_image_cumulative_distribution_function[0]);
    let ref_end_value: (u8, f64) = (255, 1.0);

    let piecewise_linear_distribution_points: [(u8, f64); 6] = [
        ref_begin_value,
        (28, ref_image_cumulative_distribution_function[28]),
        (75, ref_image_cumulative_distribution_function[75]),
        (150, ref_image_cumulative_distribution_function[150]),
        (210, ref_image_cumulative_distribution_function[210]),
        ref_end_value,
    ];
    let mut piecewise_linear_distribution: [f64; 256] = [0.0; 256];

    for (i, value) in piecewise_linear_distribution.iter_mut().enumerate() {
        for (j, point) in piecewise_linear_distribution_points.iter().enumerate() {
            if point.0 == 255 {
                *value = 1.0;
                break;
            }

            let next_point = piecewise_linear_distribution_points[j + 1];
            if (i as u8) >= point.0 && (i as u8) < next_point.0 {
                // println!("i: {}, Pm: {}, Pm+1: {}, Am: {}, Am+1: {}\n", i, point.1, next_point.1, point.0, next_point.0);
                *value = point.1
                    + (i as f64 - f64::from(point.0)) * (next_point.1 - point.1)
                        / f64::from(next_point.0 - point.0);
                break;
            }
        }
    }

    // println!("{}", piecewise_linear_distribution);
    // for val in piecewise_linear_distribution.iter() {
    //     println!("{}", val);
    // }
    println!();

    // create linear distribution inverse
    let mut piecewise_linear_distribution_inverse: [u8; 256] = [0; 256];

    for ((i, dist_val), dist_val_inverse) in piecewise_linear_distribution
        .iter()
        .enumerate()
        .zip(piecewise_linear_distribution_inverse.iter_mut())
    {
        // println!("Pl[0]: {}", piecewise_linear_distribution[0]);
        if *dist_val < piecewise_linear_distribution[0] && *dist_val >= 0.0 {
            *dist_val_inverse = 0.0 as u8;
        } else if *dist_val >= 1.0 {
            *dist_val_inverse = 255.0 as u8;
        } else {
            for (j, point) in piecewise_linear_distribution_points.iter().enumerate() {
                let next_point = piecewise_linear_distribution_points[j + 1];
                if (i as u8) >= point.0 && (i as u8) <= next_point.0 {
                    // let b = *dist_val;
                    // let An = point.0;
                    let numerator = f64::from(next_point.0 - point.0);
                    let denominator = next_point.1 - point.1;
                    let b_minus_Pn = *dist_val - point.1;
                    println!("b - Pn: {}, numerator: {}, denominator: {}", b_minus_Pn, numerator, denominator);
                    let val = f64::from(point.0)
                        + (*dist_val - point.1) * f64::from(next_point.0 - point.0)
                            / (next_point.1 - point.1);
                    *dist_val_inverse = val as u8;
                    println!("i: {}, Pn: {}, Pn+1: {}, An: {}, An+1: {} val: {}", i, point.1, next_point.1, point.0, next_point.0, val);
                    break;
                }
            }
        }
    }

    println!("min pixel: {}", min(&reference_image));

    // for val in piecewise_linear_distribution_inverse.iter() {
    //     println!("{}", val);
    // }

    for pixel in new_image.pixels_mut() {
        pixel.data[0] = piecewise_linear_distribution_inverse[pixel.data[0] as usize];
    }
    // match with piecewise linear distribution

    // for i in 0..256 {
    //     image_cumulative_distribution_function[i] = f64::from(image_cumulative_histogram.values[i]) / f64::from(image_total_pixels);
    // }

    new_image
}

/// assumes pixel values from 0 to 255
fn clamp_pixel(value: i64) -> u8 {
    if value < 0 {
        value.max(i64::from(MIN_VALUE)) as u8
    } else if value > 255 {
        value.min(i64::from(MAX_VALUE)) as u8
    } else {
        value as u8
    }
}

/// min value for grayscale image
fn min(image: &GrayAlphaImage) -> u8 {
    let mut min: u8 = 255;
    for pixel in image.pixels() {
        if pixel[0] < min {
            min = pixel[0];
        } 
    }
    min
}
