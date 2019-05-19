use image::{GrayAlphaImage, GrayImage, ImageBuffer, Luma, RgbaImage};

// TODO add tests for histogram functions

pub struct RgbHistogram {
    pub red: [u32; 256],
    pub green: [u32; 256],
    pub blue: [u32; 256],
}

pub struct GrayHistogram {
    pub values: [u32; 256],
}

pub struct CumulativeRgbHistogram {
    pub red: [u32; 256],
    pub green: [u32; 256],
    pub blue: [u32; 256],
}

pub struct CumulativeGrayHistogram {
    pub values: [u32; 256],
}

pub struct IntensityHistogram([u32; 256]);

impl IntensityHistogram {
    pub fn iter(&self) -> impl Iterator<Item = &u32> {
        self.0.iter()
    }
}
pub struct CumulativeIntensityHistogram([u32; 256]);

impl CumulativeIntensityHistogram {
    pub fn iter(&self) -> impl Iterator<Item = &u32> {
        self.0.iter()
    }
}

pub fn intensity_histogram(intensity_data: &[u8]) -> IntensityHistogram {
    let mut histogram: [u32; 256] = [0; 256];
    for intensity in intensity_data.iter() {
        histogram[*intensity as usize] + 1;
    }
    IntensityHistogram(histogram)
}

pub fn cumulative_intensity_histogram(intensity_data: &[u8]) -> CumulativeIntensityHistogram {
    let histogram = intensity_histogram(intensity_data);
    let mut cumulative_histogram: [u32; 256] = [0; 256];
    let mut cumulative_value = 0;

    for (hist_value, cumulative_hist_value) in histogram.iter().zip(cumulative_histogram.iter_mut())
    {
        cumulative_value = *hist_value + cumulative_value;
        *cumulative_hist_value = cumulative_value;
    }

    CumulativeIntensityHistogram(cumulative_histogram)
}

// TODO add test for histogram functions
// Check if sum of all values equals image height times image width
pub fn cumulative_gray_histogram(gray_image: &GrayAlphaImage) -> CumulativeGrayHistogram {
    let mut cumulative_histogram = [0_u32; 256];
    let gray_histogram = graya_histogram(gray_image).values;
    let mut total: u32 = 0;

    for (cumulative_value, hist_value) in cumulative_histogram.iter_mut().zip(gray_histogram.iter())
    {
        total += *hist_value;
        *cumulative_value = total;
    }

    CumulativeGrayHistogram {
        values: cumulative_histogram,
    }
}

pub fn convert_to_image(
    image_width: u32,
    image_height: u32,
    gray_hist: &GrayHistogram,
) -> GrayImage {
    let hist_color = {
        let pixel = [90; 1];
        Luma(pixel)
    };
    let max_hist_val: u32 = {
        let mut max = 0;
        for i in 0..255 {
            if gray_hist.values[i] > max {
                max = gray_hist.values[i];
            }
        }
        max
    };

    let raw_image_buffer = vec![255; (image_width * image_height) as usize];
    let mut image: GrayImage = ImageBuffer::from_raw(image_width, image_height, raw_image_buffer)
        .expect("white image could not be created");

    let histogram_height_max: u32 = image_height / 256 * 256;

    // determines how many pixels a pixel value will take up horizontally
    let step: u32 = image_width / 256;
    let hist_width: u32 = step * 256;
    let hist_begin: u32 = (image_width - hist_width) / 2;

    for i in 0..255 {
        let mut y: u32 = histogram_height_max * gray_hist.values[i] / max_hist_val;
        y = histogram_height_max - y;
        let x_pos: u32 = (i as u32) * step + hist_begin;
        for s in 0..step {
            let x: u32 = x_pos + s as u32;
            image.put_pixel(x, y, hist_color);
            // colors in every value below current x position
            for n in y..255 {
                image.put_pixel(x, n as u32, hist_color);
            }
        }
    }
    image
}

// pub fn cumulative_rgb_histogram(rgb_hist: &RgbHistogram) -> CumulativeRgbHistogram {
//     let mut cum_histogram = CumulativeRgbHistogram {
//         red: [0; 256],
//         green: [0; 256],
//         blue: [0; 256],
//     };
//
//     let mut red_sum: u32 = 0;
//     let mut green_sum: u32 = 0;
//     let mut blue_sum: u32 = 0;
//
//     for i in 0..256 {
//         red_sum += rgb_hist.red[i];
//         green_sum += rgb_hist.green[i];
//         blue_sum += rgb_hist.blue[i];
//         cum_histogram.red[i] = red_sum;
//         cum_histogram.green[i] = green_sum;
//         cum_histogram.blue[i] = blue_sum;
//     }
//
//     cum_histogram
// }

pub fn rgba_histogram(image: &RgbaImage) -> RgbHistogram {
    let mut histogram = RgbHistogram {
        red: [0; 256],
        green: [0; 256],
        blue: [0; 256],
    };
    for pixel in image.pixels() {
        histogram.red[pixel[0] as usize] += 1;
        histogram.green[pixel[1] as usize] += 1;
        histogram.blue[pixel[2] as usize] += 1;
    }
    histogram
}

pub fn rgb_histogram(image: &RgbaImage) -> RgbHistogram {
    let mut red: [u32; 256] = [0; 256];
    let mut green: [u32; 256] = [0; 256];
    let mut blue: [u32; 256] = [0; 256];

    for pixel in image.pixels() {
        red[pixel[0] as usize] += 1;
        green[pixel[1] as usize] += 1;
        blue[pixel[2] as usize] += 1;
    }
    RgbHistogram { red, green, blue }
}

pub fn graya_histogram(image: &GrayAlphaImage) -> GrayHistogram {
    let mut histogram: [u32; 256] = [0; 256];

    for pixel in image.pixels() {
        histogram[pixel[0] as usize] += 1;
    }
    GrayHistogram { values: histogram }
}

pub fn gray_histogram(image: &GrayImage) -> GrayHistogram {
    let mut histogram: [u32; 256] = [0; 256];

    for pixel in image.pixels() {
        histogram[pixel[0] as usize] += 1;
    }
    GrayHistogram { values: histogram }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{
        load_from_memory, ConvertBuffer, GenericImage, GenericImageView, GrayAlphaImage, GrayImage,
        ImageBuffer, Luma, Rgb, RgbaImage,
    };

    #[test]
    fn test_histogram() {
        let mut image: GrayAlphaImage = ImageBuffer::new(2, 2);

        image.get_pixel_mut(0, 0)[0] = 1;
        image.get_pixel_mut(0, 1)[0] = 1;
        image.get_pixel_mut(1, 0)[0] = 3;
        image.get_pixel_mut(1, 1)[0] = 4;

        let hist = graya_histogram(&image);

        assert_eq!(hist.values[1], 2);
        assert_eq!(hist.values[3], 1);
        assert_eq!(hist.values[4], 1);
    }

    //     #[test]
    //     fn test_rgb_cumulative_histogram() {
    //         // let image = GrayImage::from_raw(width: u32, height: u32, buf: Container)
    //         let mut histogram = RgbHistogram {
    //             red: [0; 256],
    //             green: [0; 256],
    //             blue: [0; 256],
    //         };
    //         for i in 0..8 {
    //             histogram.red[i] = (i + 1) as u32;
    //         }
    //         let cumul_hist = cumulative_rgb_histogram(&histogram);
    //
    //         for i in 0..8 {
    //             println!("{}", &cumul_hist.red[i]);
    //         }
    //         assert_eq!(cumul_hist.red[0], 1);
    //         assert_eq!(cumul_hist.red[1], 3);
    //         assert_eq!(cumul_hist.red[2], 6);
    //         assert_eq!(cumul_hist.red[3], 10);
    //         assert_eq!(cumul_hist.red[4], 15);
    //         assert_eq!(cumul_hist.red[5], 21);
    //         assert_eq!(cumul_hist.red[6], 28);
    //         assert_eq!(cumul_hist.red[7], 36);
    //     }
    //
    // #[test]
    // fn test_gray_cumulative_histogram() {
    //     let mut histogram = GrayHistogram { values: [0; 256] };
    //     for i in 0..8 {
    //         histogram.values[i] = (i + 1) as u32;
    //     }
    //     let cumul_hist = cumulative_gray_histogram(&histogram);

    //     for i in 0..8 {
    //         println!("{}", &cumul_hist.values[i]);
    //     }
    //     assert_eq!(cumul_hist.values[0], 1);
    //     assert_eq!(cumul_hist.values[1], 3);
    //     assert_eq!(cumul_hist.values[2], 6);
    //     assert_eq!(cumul_hist.values[3], 10);
    //     assert_eq!(cumul_hist.values[4], 15);
    //     assert_eq!(cumul_hist.values[5], 21);
    //     assert_eq!(cumul_hist.values[6], 28);
    //     assert_eq!(cumul_hist.values[7], 36);
    // }
}
