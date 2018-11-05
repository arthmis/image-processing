extern crate image; 

use image::{GenericImage, ImageBuffer, GenericImageView, RgbaImage, GrayAlphaImage, ConvertBuffer};

struct RgbaHistogram {
    red: [u32; 256],
    green: [u32; 256],
    blue: [u32; 256],
}

struct GrayHistogram {
    values: [u32; 256],
}

impl RgbaHistogram {
    fn new() -> RgbaHistogram{
        RgbaHistogram {
            red: [0; 256],
            green: [0; 256],
            blue: [0; 256],
        }
    }
}

impl GrayHistogram {
    fn new() -> GrayHistogram{
        GrayHistogram {
            values: [0; 256], 
        }
    }
}

fn main() {
    let img: GrayAlphaImage = image::open("images/london-bridge.jpg")
        .expect("Image not found").to_rgba().convert();

    // let img: GrayAlphaImage = image::open("images/london-bridge.jpg")
    //     .expect("Image not found").to_luma_alpha();


    let hist = gray_histogram(&img);

    img.save("images/gray-london-bridge.jpg").expect("directory or file not found");
    

    // let mean = get_mean(&img);
    // let variance = get_variance(&img);

    // println!("mean: {}\nvariance: {}", mean, variance); 
}

fn rgba_histogram(image: &RgbaImage) -> RgbaHistogram {
    let mut histogram = RgbaHistogram::new();
    for pixel in image.pixels() {
        histogram.red[pixel[0] as usize] += 1;
        histogram.green[pixel[1] as usize] += 1;
        histogram.blue[pixel[2] as usize] += 1;
    }
    histogram
}

fn gray_histogram(image: &GrayAlphaImage) -> GrayHistogram {
    let mut histogram = GrayHistogram::new();
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


// #[cfg(test)]
// mod tests {
//     use super::*;
//     use image::{GenericImage, ImageBuffer, GenericImageView, RgbaImage, GrayAlphaImage, ConvertBuffer, GrayImage, Luma, Rgb, load_from_memory};
//     #[test]
//     fn test_histogram() {
//         let image = load_from_memory(&[1u8, 2u8, 3u8, 2u8, 1u8]).unwrap();
//         let hist = gray_histogram(&image);

//         assert_eq!(hist[0], 0);
//         assert_eq!(hist[1], 2);
//         assert_eq!(hist[2], 2);
//         assert_eq!(hist[3], 1);
// }
// }