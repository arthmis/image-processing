use image::GenericImageView;
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
    pub hue: Vec<u16>,
    saturation: Vec<f32>,
    intensity: Vec<u8>,
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
        let mut hue = Vec::with_capacity(capacity);
        for i in 0..capacity {
            hue.push(i as u16);
        }
        HSV {
            // hue: Vec::with_capacity(capacity),
            hue,
            saturation: Vec::with_capacity(capacity),
            intensity: Vec::with_capacity(capacity),
            width, 
            height,
        }
    }

    pub fn from_image<I>(image: I) -> Self 
    where 
        I: GenericImageView,
    {
        unimplemented!();
    }

}

/// Iterators 
impl HSV { 
    pub fn enumerate_hues(&self) -> impl Iterator<Item = ((usize, usize), &u16)> {
        let width = self.width as usize;
        
        self.hue.iter()
            .enumerate()
            .map(move |(index, hue)| {
                let x = index % width;
                let y = index / width;
                ((x, y), hue)
            })
    }
    pub fn hues(&self) -> impl Iterator<Item = &u16> {
        // HueIter {
        //     hues: &self.hue,
        //     x: 0,
        //     y: 0,
        //     width: self.width,
        //     height: self.height,
        // }
        self.hue.iter()
    }

    pub fn hues_mut(&mut self) -> impl Iterator<Item = &mut u16> {
        self.hue.iter_mut()
    }

    pub fn saturations(&self) -> impl Iterator<Item = &f32> {
        self.saturation.iter()
    }

    pub fn saturations_mut(&mut self) -> impl Iterator<Item = &mut f32> {
        self.saturation.iter_mut()
    }

    pub fn intensities(&self) -> impl Iterator<Item = &u8> {
        self.intensity.iter()
    }

    pub fn intensities_mut(&mut self) -> impl Iterator<Item = &mut u8> {
        self.intensity.iter_mut()
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

// /// Iterators
// impl Iterator for HSV {
//     type Item = (u16, f32, u8);

//     fn next(&mut self) -> Option<Self::Item> { unimplemented!() }

// }
