use image::{RgbaImage, GrayAlphaImage, GrayImage};

pub struct RgbHistogram {
    pub red: [u32; 256],
    pub green: [u32; 256],
    pub blue: [u32; 256],
}

pub struct GrayHistogram {
    pub values: [u32; 256],
}

pub struct CumuRgbHistogram {
    pub red: [u32; 256],
    pub green: [u32; 256],
    pub blue: [u32; 256],
}

pub struct CumuGrayHistogram {
    pub values: [u32; 256],
}

pub fn cumulative_gray_histogram(
    gray_hist: &GrayHistogram) -> CumuGrayHistogram 
{
    let mut cum_histogram = CumuGrayHistogram {
        values: [0_u32; 256],
    };
    let mut sum: u32 = 0; 
    for i in 0..256 {
        sum += gray_hist.values[i];
        cum_histogram.values[i] = sum;
    }
    cum_histogram
}

pub fn cumulative_rgb_histogram(rgb_hist: &RgbHistogram) -> CumuRgbHistogram {
    let mut cum_histogram = CumuRgbHistogram {
            red: [0; 256],
            green: [0; 256],
            blue: [0; 256],
    };

    let mut red_sum: u32 = 0; 
    let mut green_sum: u32 = 0; 
    let mut blue_sum: u32 = 0; 

    for i in 0..256 { 
        red_sum += rgb_hist.red[i];
        green_sum += rgb_hist.green[i];
        blue_sum += rgb_hist.blue[i];
        cum_histogram.red[i] = red_sum;
        cum_histogram.green[i] = green_sum;
        cum_histogram.blue[i] = blue_sum;
    }

    cum_histogram
}

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

pub fn graya_histogram(image: &GrayAlphaImage) -> GrayHistogram {
    let mut histogram = GrayHistogram {
        values: [0; 256], 
    };

    for pixel in image.pixels() {
        histogram.values[pixel[0] as usize] += 1;
    }
    histogram
}

pub fn gray_histogram(image: &GrayImage) -> GrayHistogram {
    let mut histogram = GrayHistogram {
            values: [0; 256], 
        };
    for pixel in image.pixels() {
        histogram.values[pixel[0] as usize] += 1;
    }
    histogram
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::{GenericImage, ImageBuffer, GenericImageView, RgbaImage, GrayAlphaImage, ConvertBuffer, GrayImage, Luma, Rgb, load_from_memory};
    
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

    #[test]
    fn test_rgb_cumulative_histogram() {
        // let image = GrayImage::from_raw(width: u32, height: u32, buf: Container)
        let mut histogram = RgbHistogram {
            red: [0; 256],
            green: [0; 256],
            blue: [0; 256],
        };
        for i in 0..8 {
            histogram.red[i] = (i + 1) as u32;
        }
        let cumul_hist = cumulative_rgb_histogram(&histogram);
        
        for i in 0..8{
            println!("{}", &cumul_hist.red[i]);
        }
        assert_eq!(cumul_hist.red[0], 1);
        assert_eq!(cumul_hist.red[1], 3);
        assert_eq!(cumul_hist.red[2], 6);
        assert_eq!(cumul_hist.red[3], 10);
        assert_eq!(cumul_hist.red[4], 15);
        assert_eq!(cumul_hist.red[5], 21);
        assert_eq!(cumul_hist.red[6], 28);
        assert_eq!(cumul_hist.red[7], 36);
    }

        #[test]
    fn test_gray_cumulative_histogram() {
        let mut histogram = GrayHistogram {
            values: [0; 256],
        };
        for i in 0..8 {
            histogram.values[i] = (i + 1) as u32;
        }
        let cumul_hist = cumulative_gray_histogram(&histogram);
        
        for i in 0..8{
            println!("{}", &cumul_hist.values[i]);
        }
        assert_eq!(cumul_hist.values[0], 1);
        assert_eq!(cumul_hist.values[1], 3);
        assert_eq!(cumul_hist.values[2], 6);
        assert_eq!(cumul_hist.values[3], 10);
        assert_eq!(cumul_hist.values[4], 15);
        assert_eq!(cumul_hist.values[5], 21);
        assert_eq!(cumul_hist.values[6], 28);
        assert_eq!(cumul_hist.values[7], 36);
    }
}