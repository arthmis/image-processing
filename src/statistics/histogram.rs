use image::{RgbaImage, GrayAlphaImage, GrayImage, ImageBuffer, Luma};

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

pub fn draw_histogram(image_width: u32, image_height: u32, gray_hist: &GrayHistogram) -> GrayImage {
    let hist_color = {
        let pixel = [90; 1];
        let pixel = Luma(pixel); 
        pixel
    };
    let max_hist_val = {
        let mut max = 0;
        for i in 0..255 {
            if gray_hist.values[i] > max {
                max = gray_hist.values[i];
            }
        }
        max
    };

    // let draw_hist = || {    let raw_image_buffer = vec![255; (image_width * image_height) as usize];
    // let mut image: GrayImage = ImageBuffer::from_raw(
    //     image_width, image_height, raw_image_buffer)
    //         .expect("white image could not be created"); 
    // let step = image_width / 256;
    // for i in 0..255 {
    //     let mut y = 255 * gray_hist.values[i] / max_hist_val;
    //     y = 255 - y; 
    //     image.put_pixel(i as u32, y as u32, hist_color);
    //     for n in y..255 {
    //         image.put_pixel(i as u32, n as u32, hist_color);
    //     } 
    // }
    // image.save(file).expect("couldn't save png image");
    // };

    // draw_hist(); 
    
    let raw_image_buffer = vec![255; (image_width * image_height) as usize];
    let mut image: GrayImage = ImageBuffer::from_raw(
        image_width, image_height, raw_image_buffer)
            .expect("white image could not be created"); 

    let hist_height_max = image_height / 256 * 256; 

    let step = image_width / 256;
    println!("{}", step);
    let hist_width = step * 256; 
    let hist_begin = 0 + (image_width - hist_width) / 2;  

    for i in 0..255 {
        let mut y = hist_height_max * gray_hist.values[i] / max_hist_val;
        y = hist_height_max - y; 
        let x_pos = (i as u32) * step + hist_begin;
        for s in 0..step {
            let x = x_pos + s as u32;
            image.put_pixel(x, y, hist_color);
            // println!("{} {} {}", x, s, y);
            for n in y..255 {
                image.put_pixel(x, n as u32, hist_color);
            } 
        }
    }
    image
    // image.save(file).expect("couldn't save png image");
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
