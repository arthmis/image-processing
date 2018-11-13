extern crate image; 

use image::{ImageBuffer, RgbaImage, GrayAlphaImage, ConvertBuffer, LumaA, Pixel, GrayImage, GenericImage};

struct RgbHistogram {
    red: [u32; 256],
    green: [u32; 256],
    blue: [u32; 256],
}

struct GrayHistogram {
    values: [u32; 256],
}

struct CumRgbHistogram {
    red: [u32; 256],
    green: [u32; 256],
    blue: [u32; 256],
}

struct CumGrayHistogram {
    values: [u32; 256],
}

fn main() {

    let img = image::open("images/london-bridge.jpg")
        .expect("Image not found").to_rgba();

    rgba_histogram(&img);

    // let mut image: GrayAlphaImage = ImageBuffer::new(2, 2);

    // for pix in image.pixels_mut() {
    //     let channels = pix.channels_mut();
    //     channels[0] = 100;
    //     channels[1] = 255;  
    // }

    // let hist = gray_histogram(&image);
    // for i in 0..255 {
    //     println!("{}: {}", i,  &hist.values[i]);
    // }
    // let hist = lumaA_histogram(&img);

    // img.save("images/gray-london-bridge.jpg").expect("directory or file not found");
    

    // let mean = get_mean(&img);
    // let variance = get_variance(&img);

    // println!("mean: {}\nvariance: {}", mean, variance); 
}

fn cumulative_gray_histogram(gray_hist: &GrayHistogram) -> CumGrayHistogram {
    let mut cum_histogram = CumGrayHistogram {
        values: [0_u32; 256],
    };
    let mut sum: u32 = 0; 
    for i in 0..256 {
        sum += gray_hist.values[i];
        cum_histogram.values[i] = sum;
    }
    cum_histogram
}

fn cumulative_rgb_histogram(rgb_hist: &RgbHistogram) -> CumRgbHistogram {
    let mut cum_histogram = CumRgbHistogram {
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

fn rgba_histogram(image: &RgbaImage) -> RgbHistogram {
    let mut histogram = RgbHistogram {
            red: [0; 256],
            green: [0; 256],
            blue: [0; 256],
    };
    // let mut histogram = [[0_u32; 256]; 3];
    // for pixel in image.pixels() {
    //     histogram[0][pixel[0] as usize] += 1;
    //     histogram[1][pixel[1] as usize] += 1;
    //     histogram[2][pixel[2] as usize] += 1;
    // }
    for pixel in image.pixels() {
        histogram.red[pixel[0] as usize] += 1;
        histogram.green[pixel[1] as usize] += 1;
        histogram.blue[pixel[2] as usize] += 1;
    }
    histogram
}

fn rgb_histogram(image: &RgbaImage) -> RgbHistogram {
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

fn graya_histogram(image: &GrayAlphaImage) -> GrayHistogram {
    let mut histogram = GrayHistogram {
        values: [0; 256], 
    };

    for pixel in image.pixels() {
        histogram.values[pixel[0] as usize] += 1;
    }
    histogram
}

fn gray_histogram(image: &GrayImage) -> GrayHistogram {
    let mut histogram = GrayHistogram {
            values: [0; 256], 
        };
    for pixel in image.pixels() {
        histogram.values[pixel[0] as usize] += 1;
    }
    histogram
}

fn get_mean(image: &RgbaImage) -> f64 {
    let image_iter = image.pixels(); 
    let mut mean: f64 = 0.0;
    for pixel in image_iter {
        mean += (f64::from(pixel[0]) + f64::from(pixel[1]) + 
                f64::from(pixel[2])) / 3.0; 
    }
    mean /= f64::from(image.width()) * f64::from(image.height()); 

    mean
}

fn get_variance(image: &RgbaImage) -> f64 {
    let mean = get_mean(&image); 
    let mut variance: f64 = 0.0;
    let image_iter = image.pixels(); 
    for pixel in image_iter {
        let pixel_average = (f64::from(pixel[0]) + f64::from(pixel[1]) + 
                f64::from(pixel[2])) / 3.0;
        variance += (pixel_average - mean).powi(2); 
    }
    variance /= f64::from(image.width()) * f64::from(image.height());
    
    variance
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