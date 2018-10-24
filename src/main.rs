extern crate image; 

use image::{GenericImage, ImageBuffer, GenericImageView, RgbaImage, GrayAlphaImage};

struct ColorHistogram {
    red: [u32; 256],
    green: [u32; 256],
    blue: [u32; 256],
}

struct GrayScaleHistogram {
    values: [u8; 256],
}

impl ColorHistogram {
    fn new() -> ColorHistogram{
        ColorHistogram {
            red: [0; 256],
            green: [0; 256],
            blue: [0; 256],
        }
    }
}

impl GrayScaleHistogram {
    fn new() -> GrayScaleHistogram{
        GrayScaleHistogram {
            values: [0; 256], 
        }
    }
}

fn main() {
    let img = image::open("images/london-bridge.jpg").unwrap().to_rgba();

    let hist = get_color_histogram(&img);
}

fn get_color_histogram(image: &RgbaImage) -> ColorHistogram {
    let image_iter = image.pixels(); 
    let mut histogram = ColorHistogram::new();
    for pixel in image_iter {
        histogram.red[pixel[0] as usize] += 1;
        histogram.green[pixel[1] as usize] += 1;
        histogram.blue[pixel[2] as usize] += 1;
    }
    for i in 0..256 {
        println!("r: {} g: {} b: {}",
            histogram.red[i], 
            histogram.green[i], 
            histogram.blue[i]);
    }
    histogram
}

fn get_histogram(image: &GrayAlphaImage) -> GrayScaleHistogram {
    let image_iter = image.pixels(); 
    let mut histogram = GrayScaleHistogram::new();
    for pixel in image_iter {
        histogram.values[pixel[0] as usize] += 1;
    }
    for i in 0..256 {
        println!("values: {}", histogram.values[i]);
    }
    histogram
}
