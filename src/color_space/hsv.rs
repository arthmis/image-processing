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

    pub fn from_image<I>(image: RgbaImage) -> Self {
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
                    let denominator = ((red + green).powi(2) + (red - blue) * (green - blue)).powf(0.5);
                    (numerator / denominator).acos()
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
                saturation_data.push(saturation.round());
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
fn min_rgb(red: f32, green: f32, blue: f32) -> f32 {
    blue.min(red.min(green))
}
// /// Iterators
// impl Iterator for HSV {
//     type Item = (u16, f32, u8);

//     fn next(&mut self) -> Option<Self::Item> { unimplemented!() }

// }
