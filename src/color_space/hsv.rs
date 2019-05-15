use image::RgbaImage;
// pub struct HueIter<'a> {
//     hues: &'a [u16],
//     x: usize = 0,
//     y: usize,
//     width: u32,
//     height: u32,
// }

// impl<'a> Iterator for HueIter<'a> {
//     type Item = &'a u16;

//     // #[inline(always)]
//     fn next(&mut self) -> Option<Self::Item> {
//         if self.y >= self.height as usize {
//             None
//         } else {
//             let x = self.x;
//             let y = self.y;

//             if self.x + 1 >= self.width as usize {
//                 self.y += 1;
//                 self.x = 0;
//             } else {
//                 self.x += 1;
//             }

//             Some(&self.hues[x + (self.width as usize * y)])
//         }
//     }
// }

pub struct HSV {
    hue_data: Vec<u16>,
    saturation_data: Vec<f32>,
    intensity_data: Vec<u8>,
    width: u32,
    height: u32,
}

/// Functions concerning manipulating data structures inside HSV
impl HSV {
    // this function isn't complete until I know how I will use it normally
    pub fn new(width: u32, height: u32) -> Self {
        assert!(width != 0);
        assert!(height != 0);
        let capacity = (width * height) as usize;
        let mut hue_data = Vec::with_capacity(capacity);
        for i in 0..capacity {
            hue_data.push(i as u16);
        }
        HSV {
            // hue: Vec::with_capacity(capacity),
            hue_data,
            saturation_data: Vec::with_capacity(capacity),
            intensity_data: Vec::with_capacity(capacity),
            width, 
            height,
        }
    }

    // think about adding a specific function that converts rgb pixel to hsv
    pub fn from_image(image: &RgbaImage) -> Self {
        let (width, height) = image.dimensions();

        let capacity = (width * height) as usize;
        let mut hue_data = Vec::with_capacity(capacity);
        let mut saturation_data = Vec::with_capacity(capacity);
        let mut intensity_data = Vec::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                let pixel = image.get_pixel(x, y);
                let red = pixel[0] as f32 / 255.0;
                let green = pixel[1] as f32 / 255.0;
                let blue = pixel[2] as f32 / 255.0;

                let theta: f32 = {
                    let numerator = 0.5 * ((red - green) + (red - blue));
                    let denominator = ((red - green).powf(2.0) + (red - blue) * (green - blue)).powf(0.5);
                    (numerator / denominator).acos().to_degrees()
                };

                let hue: f32 = {
                    if blue <= green {
                        theta
                    } else {
                        360.0 - theta
                    }
                };

                let saturation: f32 = {
                    1.0 - 3.0 / (red + green + blue) * min_rgb(red, green, blue)
                };

                let intensity: f32 = {
                    (red + blue + green) / 3.0
                };
                hue_data.push(hue.round() as u16);
                saturation_data.push(saturation);
                intensity_data.push((intensity * 255.0).round() as u8);
            }
        }

        HSV {
            hue_data,
            saturation_data,
            intensity_data,
            width,
            height,
        }
    }

    pub fn to_rgb_image(&self) -> RgbaImage {
        use image::ImageBuffer;

        let (width, height) = self.dimensions();
        let mut rgb_image: RgbaImage = ImageBuffer::new(width, height); 

        for y in 0..height {
            for x in 0..width {
                let x = x as usize;
                let y = y as usize;

                let rgb = Self::hsv_to_rgb((self.get_hue(x, y), self.get_saturation(x, y), self.get_intensity(x, y)));
                let pixel = rgb_image.get_pixel_mut(x as u32, y as u32);

                pixel[0] = rgb[0];
                pixel[1] = rgb[1];
                pixel[2] = rgb[2];
                pixel[3] = 255;
            }
        }
        rgb_image        
    }

    pub fn hsv_to_rgb(hsv: (u16, f32, u8)) -> [u8; 3] {
        let hue = hsv.0 as f32;
        let saturation = hsv.1;
        let brightness = hsv.2 as f32;

        if (hue as u16) < 120 {
            let blue = brightness * (1.0 - saturation);
            let red = brightness * (1.0 + (saturation * hue.to_radians().cos().to_degrees()) / (60.0 - hue).to_radians().cos().to_degrees());
            let green = 3.0 * brightness - (red + blue);
            [red.round() as u8, green.round() as u8, blue.round() as u8]
        } else if 120 <= (hue as u16) && (hue as u16) < 240 {
            let hue = hue - 120.0;
            let red = brightness * (1.0 - saturation);
            let green = brightness * (1.0 + (saturation * hue.to_radians().cos().to_degrees()) / (60.0 - hue).to_radians().cos().to_degrees());
            let blue = 3.0 * brightness - (red + green); 
            [red.round() as u8, green.round() as u8, blue.round() as u8]
        } else {
            let hue = hue - 240.0;
            let green = brightness * (1.0 - saturation);
            let blue = brightness * (1.0 + (saturation * hue.to_radians().cos().to_degrees()) / (60.0 - hue).to_radians().cos().to_degrees());
            let red = 3.0 * brightness - (green + blue); 
            [red.round() as u8, green.round() as u8, blue.round() as u8]
        }
    }
}

/// Iterators 
impl HSV { 
    pub fn enumerate_hues(&self) -> impl Iterator<Item = ((usize, usize), &u16)> {
        let width = self.width as usize;
        
        self.hue_data.iter()
            .enumerate()
            .map(move |(index, hue)| {
                let x = index % width;
                let y = index / width;
                ((x, y), hue)
            })
    }
    pub fn hues(&self) -> impl Iterator<Item = &u16> {
        // HueIter {
        //     hues: &self.hue_data,
        //     x: 0,
        //     y: 0,
        //     width: self.width,
        //     height: self.height,
        // }
        self.hue_data.iter()
    }

    pub fn hues_mut(&mut self) -> impl Iterator<Item = &mut u16> {
        self.hue_data.iter_mut()
    }

    pub fn saturations(&self) -> impl Iterator<Item = &f32> {
        self.saturation_data.iter()
    }

    pub fn saturations_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.saturation_data.iter_mut()
    }

    pub fn intensities(&self) -> impl Iterator<Item = &u8> {
        self.intensity_data.iter()
    }

    pub fn intensities_mut(&mut self) -> impl Iterator<Item = &mut u8> {
        self.intensity_data.iter_mut()
    }

}

impl HSV {
    pub fn get_hue(&self, x: usize, y: usize) -> u16 {
        self.hue_data[x + (self.width as usize * y)]
    }

    pub fn get_saturation(&self, x: usize, y: usize) -> f32 {
        self.saturation_data[x + (self.width as usize * y)]
    }
    
    pub fn get_intensity(&self, x: usize, y: usize) -> u8 {
        self.intensity_data[x + (self.width as usize * y)]
    }
}

/// Methods concerning image dimensions
impl HSV {
    pub fn width(&self) -> u32 {
        self.width
    }
    
    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }
     
    pub fn len(&self) -> u32 {
        self.width * self.height
    }
}


// Utility functions
pub fn min_rgb(red: f32, green: f32, blue: f32) -> f32 {
    blue.min(red.min(green))
}
// /// Iterators
// impl Iterator for HSV {
//     type Item = (u16, f32, u8);

//     fn next(&mut self) -> Option<Self::Item> { unimplemented!() }

// }


#[cfg(test)]
mod tests {
    use image::RgbaImage;
    use super::*;
    // going have to complete this, figuring out the exact values is a pain
    #[test] 
    fn rgb_to_hsv() {
        let raw_data = vec![
            25, 33, 44, 255,
            88, 21, 30, 255,
            99, 0, 63, 255,
            156, 200, 185, 255,
        ];
        let image: RgbaImage = image::ImageBuffer::from_raw(2, 2, raw_data).unwrap(); 
        let hsv_image = HSV::from_image(&image);
        for ((hue, saturation), brightness) in hsv_image.hues()
            .zip(hsv_image.saturations())
            .zip(hsv_image.intensities()) 
        {
            println!("hue: {}, saturation: {}, brightness: {}", hue, saturation, brightness);
        }
    }
    
    #[test]
    fn test_min_rgb() {
        let (red, green, blue) = (44.0, 33.0, 25.0); 
        let min = min_rgb(red, green, blue);
        assert_eq!(min, 25.0);
    }
}