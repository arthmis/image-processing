//! Operations that only deal with one pixel

use image::{GrayAlphaImage, LumaA};

const MAX_VALUE: u8 = 255;
const MIN_VALUE: u8 = 0;

// TODO add contrast mut function

/// contrast adjustments
/// values from 0-99 decrease contrast and 100-200 increase contrast
pub fn contrast(image: &GrayAlphaImage, contrast: u8) -> GrayAlphaImage {
    let contrast: f32 = {
        let normalized_contrast = if contrast > 199 {
            200.0
        } else {
            f32::from(contrast)
        };
        normalized_contrast * 2.0 / 200.0
    };

    let mut new_image = image.clone();
    for pixel in new_image.pixels_mut() {
        let new_pixel_value = (f32::from(pixel.data[0 as usize]) - 128.0) * contrast + 128.0;
        let new_pixel_value = new_pixel_value.round() as i64;
        let new_pixel_value = clamp_pixel(new_pixel_value);
        pixel.data[0 as usize] = new_pixel_value;
    }

    new_image
}

// also consider making alternative that accepts hi and lo range for percentage ranges of pixels
// decide which auto contrast function to keep or keep both and see if there is use for the 
// unmodified version

/// saturates percentage of pixels from the bottom and top of the image intensity spectrum
/// then linearly distributes the pixels within that spectrum 
pub fn modified_auto_contrast_mut (image: &mut GrayAlphaImage) {
    use crate::statistics::histogram::cumulative_gray_histogram;

    let (percentage_min, percentage_max) = (0.01_f64, 0.01_f64);
    let cumulative_histogram = cumulative_gray_histogram(&image);
    let pixels_total: f64 = {
        let (width, height) = image.dimensions();
        (width * height) as f64
    };

    let image_min_value: u8 = {
        let mut minimum: u8 = MAX_VALUE;
        for (i, value) in cumulative_histogram.values.iter().enumerate() {
            if *value as f64 >= pixels_total * percentage_min {
                if (i as u8) < minimum {
                    minimum = i as u8;
                }
            }
        }
        // dbg!(minimum);
        minimum
    };

    let image_max_value: u8 = {
        let mut maximum: u8 = MIN_VALUE;
        for (i, value) in cumulative_histogram.values.iter().enumerate() {
            if *value as f64 <= pixels_total * (1.0 - percentage_max) {
                if (i as u8) > maximum {
                    maximum = i as u8;
                }
            }
        }
        // dbg!(maximum);
        maximum
    };

    for pixel in image.pixels_mut() {
        let pixel_value = f64::from(pixel.data[0]);

        if pixel_value <= image_min_value as f64 {
            pixel.data[0] = MIN_VALUE;
        } else if pixel_value >= image_max_value as f64 {
            pixel.data[0] = MAX_VALUE;
        } else {
            let new_pixel_value = f64::from(MIN_VALUE)
                + (pixel_value - image_min_value as f64) 
                * f64::from(MAX_VALUE)
                / (image_max_value - image_min_value) as f64;
            let new_pixel_value = new_pixel_value.round() as i64;
            let new_pixel_value = clamp_pixel(new_pixel_value);
            pixel.data[0] = new_pixel_value;
        }

    }
}

/// See modified_auto_contrast_mut
pub fn modified_auto_contrast(image: &GrayAlphaImage) -> GrayAlphaImage {
    let mut new_image = image.clone();
    modified_auto_contrast_mut(&mut new_image);
    new_image
}

/// automatic contrast adjustment
// TODO continue tweaking this to work better and use modified auto contrast
// consider creating an alternate function that accepts lower and upper bound for
// contrast range
pub fn auto_contrast(image: &GrayAlphaImage) -> GrayAlphaImage {
    let mut new_image = image.clone();

    let min_pixel = {
        let mut min: u8 = MAX_VALUE;
        for pixel in new_image.pixels() {
            if pixel.data[0 as usize] < min {
                min = pixel.data[0 as usize];
            }
        }
        f64::from(min)
    };

    let max_pixel = {
        let mut max: u8 = MIN_VALUE;
        for pixel in new_image.pixels() {
            if pixel.data[0 as usize] > max {
                max = pixel.data[0 as usize];
            }
        }
        f64::from(max)
    };

    for pixel in new_image.pixels_mut() {
        let pixel_value = f64::from(pixel.data[0]);
        let new_pixel_value = f64::from(MIN_VALUE)
            + (pixel_value - min_pixel) * f64::from(MAX_VALUE)
                / (max_pixel - min_pixel);
        let new_pixel_value = new_pixel_value.round() as i64;
        let new_pixel_value = clamp_pixel(new_pixel_value);
        pixel.data[0] = new_pixel_value;
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
        let new_pixel_value = i64::from(pixel.2.data[0 as usize]) + brightness;
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
// and add documentation for it
// make it possible to pass in your own distribution as a histogram of any size less than 256
// make it generic over the control points that are parameters for this function
// make an inplace version of this function
// rename variables appropriately
pub fn match_piecewise_linear_histogram(image: &GrayAlphaImage, reference_image: &GrayAlphaImage) -> GrayAlphaImage {
    use crate::statistics::histogram::cumulative_gray_histogram;
    use crate::statistics::histogram::graya_histogram;

    let mut new_image = image.clone();

    // calculate cumulative distribution functions for both images
    let image_cumulative_histogram: [u32; 256] = cumulative_gray_histogram(&new_image).values;
    let ref_image_cumulative_histogram: [u32; 256] =
        cumulative_gray_histogram(&reference_image).values;

    let mut image_cumulative_distribution_function: [f64; 256] = [0.0; 256];
    let mut ref_image_cumulative_distribution_function: [f64; 256] = [0.0; 256];

    let image_total_pixels = image_cumulative_histogram[255] as f64;
    let ref_image_total_pixels = ref_image_cumulative_histogram[255] as f64;

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

    // create piecewise linear distribution for reference image
    let piecewise_linear_distribution_points: [(u8, f64); 6] = [
        (0, ref_image_cumulative_distribution_function[0]),
        (28, ref_image_cumulative_distribution_function[28]),
        (75, ref_image_cumulative_distribution_function[75]),
        (150, ref_image_cumulative_distribution_function[150]),
        (210, ref_image_cumulative_distribution_function[210]),
        (255, 1.0),
    ];
    
    let mut piecewise_linear_distribution: [f64; 256] = [0.0; 256];

    for (i, value) in piecewise_linear_distribution.iter_mut().enumerate() {
        if i == 255 {
            *value = 1.0;
            // break;
        } else {
            for (j, point) in piecewise_linear_distribution_points.iter().enumerate().rev() {
                if point.0 <= i as u8 {
                    let next_point = piecewise_linear_distribution_points[j + 1];
                    *value = point.1
                        + (i as f64 - f64::from(point.0)) 
                        * (next_point.1 - point.1)
                        / f64::from(next_point.0 - point.0);
                    break;
                }
            }
        }
    }

    // create linear distribution inverse
    let mut piecewise_linear_distribution_inverse: [u8; 256] = [0; 256];

    for ((i, inverse_value), b) in piecewise_linear_distribution_inverse.iter_mut().enumerate().zip(image_cumulative_distribution_function.iter()) {
        if *b <= piecewise_linear_distribution_points[0].1 as f64 {
            *inverse_value = 0;
        } else if *b >= 1.0 {
            *inverse_value = 255;
        } else {
            for (j, point) in piecewise_linear_distribution_points.iter().enumerate().rev() {

                if point.1 <= *b {
                    let next_point = piecewise_linear_distribution_points[j + 1];
                    *inverse_value = (point.0 as f64
                        + (*b - point.1) 
                        * f64::from(next_point.0 - point.0)
                        / (next_point.1 - point.1))
                        .round() as u8;
                    break;
                }
            }
        } 
    }

    for pixel in new_image.pixels_mut() {
        let old_pixel_value = pixel.data[0];
        pixel.data[0] = piecewise_linear_distribution_inverse[old_pixel_value as usize];
    }

    new_image
}

pub fn histogram_matching(image: &mut GrayAlphaImage, reference_image: &GrayAlphaImage) {
    // todo!();
}

// improve this function to be generic and clamp and return whichever numeric type
// the user wants
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
