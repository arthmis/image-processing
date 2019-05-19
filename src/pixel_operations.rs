//! Operations that only deal with one pixel

use image::{GenericImage, Pixel, Primitive, RgbaImage};
use num_traits::cast::NumCast;

const MAX_VALUE: u8 = 255;
// const MIN_VALUE: u8 = 0;
/// Weighted values to turn rgb to luma
const RED_WEIGHT: f32 = 0.299;
const GREEN_WEIGHT: f32 = 0.587;
const BLUE_WEIGHT: f32 = 0.114;

pub fn clamp<T: Primitive>(value: T, min: T, max: T) -> T {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

pub fn luma_weighted(red: f32, green: f32, blue: f32) -> f32 {
    red * RED_WEIGHT + green * GREEN_WEIGHT + blue * BLUE_WEIGHT
}

pub fn exposure_compensation_mut_rgb(image: &mut RgbaImage, exposure_compensation: f32) {
    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            if x == 297 && y == 165 {
                let pixel = image.get_pixel(x, y);
                dbg!(pixel[0]);
                dbg!(pixel[1]);
                dbg!(pixel[2]);
            }
            image.get_pixel_mut(x, y).apply_without_alpha(|value| {
                let value: f32 = NumCast::from(value).expect("failed cast from u8 to f64");
                let new_value = value * (2.0_f32.powf(exposure_compensation));
                let new_value = clamp(new_value.round(), 0.0, 255.0);
                NumCast::from(new_value.round()).expect("failed cast from f64 to u8")
            });
            // let pixel = image.get_pixel_mut(x, y).channels_mut();
            // // println!("{:?}", pixel[0]);
            // let red: f32 = NumCast::from(pixel[0]).unwrap();
            // let green: f32 = NumCast::from(pixel[1]).unwrap();
            // let blue: f32 = NumCast::from(pixel[2]).unwrap();
            // let exposure_transformation = 2.0_f32.powf(exposure_compensation);
            // let new_red = exposure_transformation * red * RED_WEIGHT;
            // let new_green = exposure_transformation * green * GREEN_WEIGHT;
            // let new_blue = exposure_transformation * blue * BLUE_WEIGHT;
            // pixel[0] = clamp(new_red.round(), 0.0, 255.0) as u8;
            // pixel[1] = clamp(new_green.round(), 0.0, 255.0) as u8;
            // pixel[2] = clamp(new_blue.round(), 0.0, 255.0) as u8;
        }
    }
}

// color shift will occur if the pixel becomes too bright and 1 or 2 of the
// color channels clip when converted back to rgb if they weren't clipped before
// this only occurs when increasing brightness
// will have to figure out how image editors get around this problem
pub fn exposure_compensation_mut(brightness_data: &mut [u8], exposure_compensation: f32) {
    for brightness in brightness_data.iter_mut() {
        let value: f32 = NumCast::from(*brightness).expect("failed cast from u8 to f32");
        // let value_to_add = 2.0_f32.powf(exposure_compensation);
        // let value_to_add = value * exposure_compensation;
        let mut new_value = value * (2.0_f32.powf(exposure_compensation));
        // let mut new_value = value + value_to_add;
        new_value = clamp(new_value.round(), 0.0, 255.0);
        *brightness = NumCast::from(new_value).expect("failed cast from f32 to u8");
    }
}

/// positive value increases saturation and negative decreases it
/// the input value will be used as a percentage to increase the saturation
// put an example of how to use
pub fn change_saturation_mut(saturation_data: &mut [f32], new_saturation: f32) {
    for saturation in saturation_data.iter_mut() {
        let add_value = *saturation * new_saturation;
        *saturation = clamp(*saturation + add_value, 0.0, 1.0);
    }
}

pub fn auto_contrast_mut(intensity_data: &mut [u8]) {
    use crate::statistics::histogram::cumulative_intensity_histogram;
    use std::u8;

    let pixels_total = intensity_data.len() as f32;
    let cumulative_histogram = cumulative_intensity_histogram(intensity_data);
    let (percentage_low, percentage_high) = (0.01_f32, 0.01_f32);

    let image_low: u8 = {
        let image_low = pixels_total as f32 * percentage_low;
        let mut minimum_intensity = u8::MAX;
        for (intensity, value) in cumulative_histogram.iter().enumerate() {
            if *value as f32 >= image_low {
                if (intensity as u8) < minimum_intensity {
                    minimum_intensity = intensity as u8;
                }
            }
        }
        
        minimum_intensity
    };
    

    let image_high: u8 = {
        let image_high = pixels_total as f32 * (1.0 - percentage_high);
        let mut maximum_intensity = u8::MIN;
        for (intensity, value) in cumulative_histogram.iter().enumerate() {
            if *value as f32 <= image_high {
                if (intensity as u8) > maximum_intensity {
                    maximum_intensity = intensity as u8;
                }
            }
        }
        maximum_intensity
    };

    for intensity in intensity_data.iter_mut() {
        if *intensity <= image_low as u8 {
            *intensity = u8::MIN;
        } else if *intensity >= image_high as u8 {
            *intensity = u8::MAX; 
        } else {
            let new_intensity = u8::MIN as f32
                + (*intensity as f32 - image_low as f32) * (u8::MAX - u8::MIN) as f32
                    / (image_high - image_low) as f32;
            *intensity = clamp(new_intensity.round(), u8::MIN as f32, u8::MAX as f32) as u8;
        }
    }
}
/// saturates percentage of pixels from the bottom and top of the image intensity spectrum
/// then linearly distributes the pixels within that spectrum
// pub fn modified_auto_contrast_mut(intensity_data: &mut [u8]) {
//     use crate::statistics::histogram::cumulative_gray_histogram;

//     let (percentage_min, percentage_max) = (0.01_f64, 0.01_f64);
//     let cumulative_histogram = cumulative_gray_histogram(&image);
//     let pixels_total: f64 = {
//         let (width, height) = image.dimensions();
//         (width * height) as f64
//     };

//     let image_min_value: u8 = {
//         let mut minimum: u8 = MAX_VALUE;
//         for (i, value) in cumulative_histogram.values.iter().enumerate() {
//             if *value as f64 >= pixels_total * percentage_min {
//                 if (i as u8) < minimum {
//                     minimum = i as u8;
//                 }
//             }
//         }
//         // dbg!(minimum);
//         minimum
//     };

//     let image_max_value: u8 = {
//         let mut maximum: u8 = MIN_VALUE;
//         for (i, value) in cumulative_histogram.values.iter().enumerate() {
//             if *value as f64 <= pixels_total * (1.0 - percentage_max) {
//                 if (i as u8) > maximum {
//                     maximum = i as u8;
//                 }
//             }
//         }
//         // dbg!(maximum);
//         maximum
//     };

//     for pixel in image.pixels_mut() {
//         let pixel_value = f64::from(pixel.data[0]);

//         if pixel_value <= image_min_value as f64 {
//             pixel.data[0] = MIN_VALUE;
//         } else if pixel_value >= image_max_value as f64 {
//             pixel.data[0] = MAX_VALUE;
//         } else {
//             let new_pixel_value = f64::from(MIN_VALUE)
//                 + (pixel_value - image_min_value as f64) * f64::from(MAX_VALUE)
//                     / (image_max_value - image_min_value) as f64;
//             let new_pixel_value = new_pixel_value.round() as i64;
//             let new_pixel_value = clamp_pixel(new_pixel_value);
//             pixel.data[0] = new_pixel_value;
//         }
//     }
// }

// TODO add contrast mut function

// /// contrast adjustments
// /// values from 0-99 decrease contrast and 100-200 increase contrast
// pub fn contrast(image: &GrayAlphaImage, contrast: u8) -> GrayAlphaImage {
//     let contrast: f32 = {
//         let normalized_contrast = if contrast > 199 {
//             200.0
//         } else {
//             f32::from(contrast)
//         };
//         normalized_contrast * 2.0 / 200.0
//     };

//     let mut new_image = image.clone();
//     for pixel in new_image.pixels_mut() {
//         let new_pixel_value = (f32::from(pixel.data[0 as usize]) - 128.0) * contrast + 128.0;
//         let new_pixel_value = new_pixel_value.round() as i64;
//         let new_pixel_value = clamp_pixel(new_pixel_value);
//         pixel.data[0 as usize] = new_pixel_value;
//     }

//     new_image
// }

// // also consider making alternative that accepts hi and lo range for percentage ranges of pixels
// // decide which auto contrast function to keep or keep both and see if there is use for the
// // unmodified version

// /// saturates percentage of pixels from the bottom and top of the image intensity spectrum
// /// then linearly distributes the pixels within that spectrum
// pub fn modified_auto_contrast_mut(image: &mut GrayAlphaImage) {
//     use crate::statistics::histogram::cumulative_gray_histogram;

//     let (percentage_min, percentage_max) = (0.01_f64, 0.01_f64);
//     let cumulative_histogram = cumulative_gray_histogram(&image);
//     let pixels_total: f64 = {
//         let (width, height) = image.dimensions();
//         (width * height) as f64
//     };

//     let image_min_value: u8 = {
//         let mut minimum: u8 = MAX_VALUE;
//         for (i, value) in cumulative_histogram.values.iter().enumerate() {
//             if *value as f64 >= pixels_total * percentage_min {
//                 if (i as u8) < minimum {
//                     minimum = i as u8;
//                 }
//             }
//         }
//         // dbg!(minimum);
//         minimum
//     };

//     let image_max_value: u8 = {
//         let mut maximum: u8 = MIN_VALUE;
//         for (i, value) in cumulative_histogram.values.iter().enumerate() {
//             if *value as f64 <= pixels_total * (1.0 - percentage_max) {
//                 if (i as u8) > maximum {
//                     maximum = i as u8;
//                 }
//             }
//         }
//         // dbg!(maximum);
//         maximum
//     };

//     for pixel in image.pixels_mut() {
//         let pixel_value = f64::from(pixel.data[0]);

//         if pixel_value <= image_min_value as f64 {
//             pixel.data[0] = MIN_VALUE;
//         } else if pixel_value >= image_max_value as f64 {
//             pixel.data[0] = MAX_VALUE;
//         } else {
//             let new_pixel_value = f64::from(MIN_VALUE)
//                 + (pixel_value - image_min_value as f64) * f64::from(MAX_VALUE)
//                     / (image_max_value - image_min_value) as f64;
//             let new_pixel_value = new_pixel_value.round() as i64;
//             let new_pixel_value = clamp_pixel(new_pixel_value);
//             pixel.data[0] = new_pixel_value;
//         }
//     }
// }

// /// See modified_auto_contrast_mut
// pub fn modified_auto_contrast(image: &GrayAlphaImage) -> GrayAlphaImage {
//     let mut new_image = image.clone();
//     modified_auto_contrast_mut(&mut new_image);
//     new_image
// }

// /// auto contrast enhancement by mapping the lowest and highest pixel to minimum
// /// and maximum intensity values, respectively, and restributing the rest of the pixels
// pub fn auto_contrast_mut(image: &mut GrayAlphaImage) {
//     let min_pixel = {
//         let mut min: u8 = MAX_VALUE;
//         for pixel in image.pixels() {
//             if pixel.data[0 as usize] < min {
//                 min = pixel.data[0 as usize];
//             }
//         }
//         f64::from(min)
//     };

//     let max_pixel = {
//         let mut max: u8 = MIN_VALUE;
//         for pixel in image.pixels() {
//             if pixel.data[0 as usize] > max {
//                 max = pixel.data[0 as usize];
//             }
//         }
//         f64::from(max)
//     };

//     for pixel in image.pixels_mut() {
//         let pixel_value = f64::from(pixel.data[0]);
//         let new_pixel_value = f64::from(MIN_VALUE)
//             + (pixel_value - min_pixel) * f64::from(MAX_VALUE) / (max_pixel - min_pixel);
//         let new_pixel_value = new_pixel_value.round() as i64;
//         let new_pixel_value = clamp_pixel(new_pixel_value);
//         pixel.data[0] = new_pixel_value;
//     }
// }

// /// automatic contrast adjustment
// // TODO continue tweaking this to work better and use modified auto contrast
// // consider creating an alternate function that accepts lower and upper bound for
// // contrast range
// /// see auto contrast mut
// pub fn auto_contrast(image: &GrayAlphaImage) -> GrayAlphaImage {
//     let mut new_image = image.clone();
//     auto_contrast_mut(&mut new_image);
//     new_image
// }

// // TODO add brightness mut function

// /// brightness adjustments
// /// brightness range is [-256, 255] inclusive
// /// negative values decrease brightness
// pub fn brightness(image: &GrayAlphaImage, brightness: i16) -> GrayAlphaImage {
//     let mut new_image = image.clone();
//     brightness_mut(&mut new_image, brightness);
//     new_image
// }

// use image;
// use image::Primitive;
// use image::GenericImage;
// use image::Pixel;
// // pub fn brightness_mut<I, P, S>(image: &mut GrayAlphaImage, brightness: i16)
// pub fn brightness_mut<I, P, S>(image: &mut I, brightness: i16)
// where
//     I: GenericImage<Pixel = P>,
//     P: Pixel<Subpixel = S> + 'static,
//     S: Primitive + 'static,
// {
//     // let channels = P::CHANNEL_COUNT;
//     let (width, height) = image.dimensions();

//     let brightness = if brightness > i16::from(MAX_VALUE) {
//         255_i64
//     } else if brightness < -256 {
//         -256_i64
//     } else {
//         i64::from(brightness)
//     };

//     // for pixel in image.pixels_mut() {
//     //     let new_pixel_value = i64::from(pixel.data[0]) + brightness;
//     //     let new_pixel_value: u8 = clamp_pixel(new_pixel_value);
//     //     pixel.data[0] = new_pixel_value;
//     // }
//     // let channels = image.get_pixel(0, 0).channel_count();
//     // let channels = P::CHANNEL_COUNT;
//     let channels = <P>::channel_count();
//     println!("{}", channels);
//     let (width, height) = image.dimensions();

//     // for y in 0..height {
//     //     for x in 0..width {
//     //         let new_pixel = image.get_pixel(x, y).map_with_alpha(
//     //             |b| {
//     //                 clamp_pixel((i64::from(b) + brightness) as i16);
//     //             },
//     //             |alpha| alpha,
//     //         );
//     //         image.put_pixel(x, y, new_pixel);
//     //     }
//     // }
// }

/// inverts image in place
pub fn invert_mut<I, P, S>(image: &mut I)
where
    I: GenericImage<Pixel = P>,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    let apply_color = |x| {
        let max: S = NumCast::from(MAX_VALUE).unwrap();
        max - x
    };
    let (width, height) = image.dimensions();
    for x in 0..width {
        for y in 0..height {
            image
                .get_pixel_mut(x, y)
                .apply_with_alpha(apply_color, |alpha| alpha);
        }
    }
}

pub fn invert<I, P, S>(image: &I) -> I
where
    I: GenericImage<Pixel = P> + Clone,
    P: Pixel<Subpixel = S> + 'static,
    S: Primitive + 'static,
{
    let mut new_image = image.clone();
    invert_mut(&mut new_image);
    new_image
}

// // pub fn invert_grayscale_threaded(image: &mut GrayAlphaImage) -> GrayAlphaImage {

// //     use crossbeam::thread;
// //     use num_cpus;
// //     use image::GenericImage;

// //     let cpus_num = num_cpus::get();

// //     let (width, height) = image.dimensions();
// //     thread::scope(|s| {
// //         // use chunks_mut or find a way to use it
// //         let x = 0;
// //         let mut y = 0;
// //         let y_offset = width / cpus_num as u32;
// //         for thread in 0..cpus_num {
// //             let mut sub_image = image.sub_image(x, y, width, y_offset);
// //             s.spawn(move |_| {
// //                 for pixel in sub_image.pixels_mut() {
// //                     pixel.2.data[0] = MAX_VALUE - pixel.2.data[0];
// //                 }
// //             });
// //             y += y_offset;
// //         }
// //     }).unwrap();
// //     image.clone();
// // }

// /// thresholds image in place
// pub fn threshold_mut(image: &mut GrayAlphaImage, threshold: u8) {
//     for pixel in image.pixels_mut() {
//         if pixel.data[0 as usize] < threshold {
//             pixel.data[0 as usize] = MIN_VALUE;
//         } else {
//             pixel.data[0 as usize] = MAX_VALUE;
//         }
//     }
// }

// /// thresholds image
// pub fn threshold(image: &GrayAlphaImage, threshold: u8) -> GrayAlphaImage {
//     let mut threshold_image = image.clone();
//     threshold_mut(&mut threshold_image, threshold);
//     threshold_image
// }

// // TODO create mutable version of this
// /// approximately equalize histogram
// pub fn probability_distribution_function(image: &GrayAlphaImage) -> [f64; 256] {
//     use crate::statistics::histogram::graya_histogram;

//     let width = image.width() as f64;
//     let height = image.height() as f64;
//     let total_pixels = width * height;

//     let mut probability_distribution_function = [0.0_f64; 256];
//     let histogram = graya_histogram(image).values;

//     for (hist_value, pdf_value) in histogram.iter().zip(probability_distribution_function.iter_mut()) {
//         *pdf_value = (*hist_value as f64) / total_pixels;
//     }
//     probability_distribution_function
// }

// pub fn equalize_histogram(image: &GrayAlphaImage) -> GrayAlphaImage {
//     use crate::statistics::histogram::cumulative_gray_histogram;

//     let mut new_image = image.clone();
//     let cumulative_hist = cumulative_gray_histogram(image);
//     let image_width: f64 = f64::from(new_image.width());
//     let image_height: f64 = f64::from(new_image.height());

//     let max_value: f64 = f64::from(MAX_VALUE);
//     let pixel_count = image_width * image_height;

//     for pixel in new_image.pixels_mut() {
//         let cumulative_hist_value = f64::from(cumulative_hist.values[pixel.data[0] as usize]);
//         let new_pixel_value = (cumulative_hist_value / pixel_count * max_value).floor();
//         let new_pixel_value = clamp_pixel(new_pixel_value as i64);
//         pixel.data[0] = new_pixel_value;
//     }

//     new_image
// }

// // TODO make this more efficient by calculating cumulative distribution function when(benchmark it first to be sure)
// // calculating the histogram via the function found on pg. 67
// // and add documentation for it
// // make it possible to pass in your own distribution as a histogram of any size less than 256
// // make it generic over the control points that are parameters for this function
// // make an inplace version of this function
// // rename variables appropriately
// pub fn match_piecewise_linear_histogram(
//     image: &GrayAlphaImage,
//     reference_image: &GrayAlphaImage,
// ) -> GrayAlphaImage {
//     use crate::statistics::histogram::cumulative_gray_histogram;
//     use crate::statistics::histogram::graya_histogram;
//     use image::GrayAlphaImage;

//     let mut new_image = image.clone();
//     // let mut new_image: GrayAlphaImage = image::ImageBuffer::new(image.width(), image.height());

//     // calculate cumulative distribution functions for both images
//     let image_cumulative_histogram: [u32; 256] = cumulative_gray_histogram(&image).values;
//     let reference_image_cumulative_histogram: [u32; 256] =
//         cumulative_gray_histogram(&reference_image).values;

//     let image_total_pixels = {
//         let (width, height) = image.dimensions();
//         (width * height) as f64
//     };

//     let reference_image_total_pixels = {
//         let (width, height) = reference_image.dimensions();
//         (width * height) as f64
//     };

//     let image_cumulative_distribution_function: [f64; 256] = {
//         let mut cdf: [f64; 256] = [0.0; 256];
//         for (dist_val, hist_val) in cdf.iter_mut().zip(image_cumulative_histogram.iter()) {
//             *dist_val = f64::from(*hist_val) / image_total_pixels;
//         }
//         cdf
//     };

//     let reference_image_cumulative_distribution_function: [f64; 256] = {
//         let mut cdf: [f64; 256] = [0.0; 256];

//         for (dist_val, hist_val) in cdf
//             .iter_mut()
//             .zip(reference_image_cumulative_histogram.iter())
//         {
//             *dist_val = f64::from(*hist_val) / reference_image_total_pixels;
//         }
//         cdf
//     };

//     // create piecewise linear distribution for reference image
//     let piecewise_linear_distribution_points: [(u8, f64); 6] = [
//         (0, reference_image_cumulative_distribution_function[0]),
//         (28, reference_image_cumulative_distribution_function[28]),
//         (75, reference_image_cumulative_distribution_function[75]),
//         (150, reference_image_cumulative_distribution_function[150]),
//         (210, reference_image_cumulative_distribution_function[210]),
//         (255, 1.0),
//     ];

//     // create linear distribution inverse
//     let piecewise_linear_distribution_inverse: [u8; 256] = {
//         let mut linear_inverse_distribution: [u8; 256] = [0; 256];
//         for (inverse_value, b) in linear_inverse_distribution
//             .iter_mut()
//             .zip(image_cumulative_distribution_function.iter())
//         {
//             if *b <= piecewise_linear_distribution_points[0].1 as f64 {
//                 *inverse_value = 0;
//             } else if *b >= 1.0 {
//                 *inverse_value = 255;
//             } else {
//                 for (j, point) in piecewise_linear_distribution_points
//                     .iter()
//                     .enumerate()
//                     .rev()
//                 {
//                     if point.1 <= *b {
//                         let next_point = piecewise_linear_distribution_points[j + 1];
//                         *inverse_value = (point.0 as f64
//                             + (*b - point.1) * f64::from(next_point.0 - point.0)
//                                 / (next_point.1 - point.1))
//                             .round() as u8;
//                         break;
//                     }
//                 }
//             }
//         }
//         linear_inverse_distribution
//     };

//     for pixel in new_image.pixels_mut() {
//         let old_pixel_value = pixel.data[0];
//         pixel.data[0] = piecewise_linear_distribution_inverse[old_pixel_value as usize];
//     }
//     // for pixel in image.enumerate_pixels() {
//     //     let old_pixel_value = pixel.2.data[0];
//     //     use image::GenericImage;
//     //     use image::GenericImageView;
//     //     use image::LumaA;
//     //     let new_pixel = LumaA([piecewise_linear_distribution_inverse[old_pixel_value as usize], 255]);
//     //     unsafe {
//     //         new_image.unsafe_put_pixel(pixel.0, pixel.1, new_pixel);
//     //     }
//         // pixel.data[0] = piecewise_linear_distribution_inverse[old_pixel_value as usize];
//     // }

//     new_image
// }

// pub fn match_piecewise_linear_histogram_modified(
//     image: &GrayAlphaImage,
//     reference_image: &GrayAlphaImage,
// ) -> GrayAlphaImage {
//     use crate::statistics::histogram::cumulative_gray_histogram;
//     use crate::statistics::histogram::graya_histogram;
//     use image::GrayAlphaImage;

//     // let mut new_image = image.clone();
//     let mut new_image: GrayAlphaImage = image::ImageBuffer::new(image.width(), image.height());

//     // calculate cumulative distribution functions for both images
//     // let image_cumulative_histogram: [u32; 256] = cumulative_gray_histogram(&image).values;
//     // let reference_image_cumulative_histogram: [u32; 256] =
//     //     cumulative_gray_histogram(&reference_image).values;

//     let image_total_pixels = {
//         let (width, height) = image.dimensions();
//         (width * height) as f64
//     };

//     let reference_image_total_pixels = {
//         let (width, height) = reference_image.dimensions();
//         (width * height) as f64
//     };

//     // let image_cumulative_distribution_function: [f64; 256] = {
//     //     let mut cdf: [f64; 256] = [0.0; 256];
//     //     for (dist_val, hist_val) in cdf.iter_mut().zip(image_cumulative_histogram.iter()) {
//     //         *dist_val = f64::from(*hist_val) / image_total_pixels;
//     //     }
//     //     cdf
//     // };

//     // let reference_image_cumulative_distribution_function: [f64; 256] = {
//     //     let mut cdf: [f64; 256] = [0.0; 256];

//     //     for (dist_val, hist_val) in cdf
//     //         .iter_mut()
//     //         .zip(reference_image_cumulative_histogram.iter())
//     //     {
//     //         *dist_val = f64::from(*hist_val) / reference_image_total_pixels;
//     //     }
//     //     cdf
//     // };

//     // let image_cumulative_distribution_function: [f64; 256] = {
//     //     let histogram = graya_histogram(&image).values;
//     //     let mut cdf: [f64; 256] = [0.0; 256];
//     //     let total_accumulation: f64 = 0.0;
//     //     for (hist_value, cdf_value) in histogram.iter().zip(cdf.iter_mut()) {
//     //         let accumulation = *hist_value as f64 + total_accumulation;
//     //         *cdf_value = total_accumulation / image_total_pixels;
//     //     }
//     //     cdf
//     // };

//     // let reference_image_cumulative_distribution_function: [f64; 256] = {
//     //     let histogram = graya_histogram(&reference_image).values;
//     //     let mut cdf: [f64; 256] = [0.0; 256];
//     //     let total_accumulation: f64 = 0.0;
//     //     for (hist_value, cdf_value) in histogram.iter().zip(cdf.iter_mut()) {
//     //         let accumulation = *hist_value as f64 + total_accumulation;
//     //         *cdf_value = total_accumulation / image_total_pixels;
//     //     }
//     //     cdf
//     // };
//     let image_cumulative_distribution_function = cumulative_distribution_function(&image);
//     let reference_image_cumulative_distribution_function = cumulative_distribution_function(&reference_image);

//     // create piecewise linear distribution for reference image
//     let piecewise_linear_distribution_points: [(u8, f64); 6] = [
//         (0, reference_image_cumulative_distribution_function[0]),
//         (28, reference_image_cumulative_distribution_function[28]),
//         (75, reference_image_cumulative_distribution_function[75]),
//         (150, reference_image_cumulative_distribution_function[150]),
//         (210, reference_image_cumulative_distribution_function[210]),
//         (255, 1.0),
//     ];

//     // create linear distribution inverse
//     let piecewise_linear_distribution_inverse: [u8; 256] = {
//         let mut linear_inverse_distribution: [u8; 256] = [0; 256];
//         for (inverse_value, b) in linear_inverse_distribution
//             .iter_mut()
//             .zip(image_cumulative_distribution_function.iter())
//         {
//             if *b <= piecewise_linear_distribution_points[0].1 as f64 {
//                 *inverse_value = 0;
//             } else if *b >= 1.0 {
//                 *inverse_value = 255;
//             } else {
//                 for (j, point) in piecewise_linear_distribution_points
//                     .iter()
//                     .enumerate()
//                     .rev()
//                 {
//                     if point.1 <= *b {
//                         let next_point = piecewise_linear_distribution_points[j + 1];
//                         *inverse_value = (point.0 as f64
//                             + (*b - point.1) * f64::from(next_point.0 - point.0)
//                                 / (next_point.1 - point.1))
//                             .round() as u8;
//                         break;
//                     }
//                 }
//             }
//         }
//         linear_inverse_distribution
//     };

//     // for pixel in new_image.pixels_mut() {
//     //     let old_pixel_value = pixel.data[0];
//     //     pixel.data[0] = piecewise_linear_distribution_inverse[old_pixel_value as usize];
//     // }
//     for pixel in image.enumerate_pixels() {
//         let old_pixel_value = pixel.2.data[0];
//         use image::GenericImage;
//         use image::GenericImageView;
//         use image::LumaA;
//         unsafe {
//             new_image.unsafe_put_pixel(pixel.0, pixel.1, LumaA([piecewise_linear_distribution_inverse[old_pixel_value as usize], 255]));
//         }
//     }

//     new_image
// }

// // continue to learn about this so i can explain the intuition behind it (haven't completely understood this yet)
// pub fn histogram_matching(image: &GrayAlphaImage, reference_image: &GrayAlphaImage) -> GrayAlphaImage {
//     let image_cdf = cumulative_distribution_function(&image);
//     let reference_image_cdf = cumulative_distribution_function(&reference_image);
//     let mut look_up_table: [u8; 256] = [0; 256];

//     for (i, image_cdf_value) in image_cdf.iter().enumerate() {
//         let mut j = MAX_VALUE;
//         // TODO: the statement should say j >= 0 but it overflows, figure out how to correct this
//         while j > 0 && *image_cdf_value <= reference_image_cdf[j as usize] {
//             look_up_table[i] = j;
//             j -= 1;
//         }
//     }

//     let mut new_image = image.clone();
//     for pixel in new_image.pixels_mut() {
//         let old_pixel = pixel.data[0];
//         pixel.data[0] = look_up_table[old_pixel as usize];
//     }
//     new_image
// }

// fn cumulative_distribution_function(image: &GrayAlphaImage) -> [f64; 256] {
//     let total_pixels: f64 = {
//         let (width, height) = image.dimensions();
//         (width * height) as f64
//     };

//     use crate::statistics::histogram::cumulative_gray_histogram;
//     let cumulative_histogram = cumulative_gray_histogram(&image).values;

//     let mut cdf: [f64; 256] = [0.0; 256];
//     for (hist_value, cdf_value) in cumulative_histogram.iter().zip(cdf.iter_mut()) {
//         *cdf_value = (*hist_value as f64) / total_pixels;
//     }

//     cdf
// }

// /// min value for grayscale image
// fn min(image: &GrayAlphaImage) -> u8 {
//     let mut min: u8 = 255;
//     for pixel in image.pixels() {
//         if pixel[0] < min {
//             min = pixel[0];
//         }
//     }
//     min
// }
