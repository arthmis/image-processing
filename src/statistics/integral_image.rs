use image::{GrayImage};
use crate::drawing::rectangle::Rectangle;

pub struct IntegralImage {
    width: u32,
    height: u32,
    container: Vec<u32>,
}

impl IntegralImage {
    pub fn new(width: u32, height: u32) -> IntegralImage {
        IntegralImage {
            width: width,
            height: height,
            container: vec![0; (width * height) as usize],
        }
    }

    pub fn get_point(&self, x: u32, y: u32) -> u32 {
        self.container[(x + y * self.width) as usize]
    }

    pub fn mut_point(&mut self, x: u32, y: u32, new_value: u32) {
        self.container[(x + y * self.width)  as usize] = new_value;
    }

    pub fn region_sum(&self, region: Rectangle) -> u32{
        let top_left_region = self.get_point(region.left, region.top);
        let top_region = self.get_point(region.right, region.top);
        let left_region = self.get_point(region.left, region.bottom);
        let entire_region = self.get_point(region.right, region.bottom);
 
        entire_region + top_left_region - top_region - left_region
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }
}

pub fn integral_image(image: &GrayImage) -> (IntegralImage, IntegralImage) {
    let width = image.width() + 1;
    let height = image.height() + 1;
    let mut integ_image = IntegralImage::new(width, height);
    let mut integ_image_vari = IntegralImage::new(width, height);
    unsafe {
        integ_image.container.set_len((width * height) as usize);
        integ_image_vari.container.set_len((width * height) as usize);
    }
    let mut sum: u32 = 0; 
    for y in 1..height {
        for x in 1..width {
            sum += integ_image.get_point(x-1, y);
            sum += integ_image.get_point(x, y-1);
            sum -= integ_image.get_point(x-1, y-1);
            let current_pixel = u32::from(image.get_pixel(x - 1, y - 1)[0]);
            integ_image.mut_point(x, y, sum + current_pixel);
            integ_image_vari.mut_point(x, y, sum + current_pixel.pow(2));
            sum = 0;
        }
    }
    (integ_image, integ_image_vari)
}
