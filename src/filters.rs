use image::{GrayImage};

pub fn box_3x3(image: &GrayImage) -> GrayImage {
    let mut new_image = image.clone();
    // let filter: [[u32; 3]; 3] = [[1; 3]; 3];
    let filter =
    [
        [1, 1, 1],
        [1, 1, 1],
        [1, 1, 1]
    ];

    let image_height = image.height();
    let image_width = image.width();

    for pixel in image.enumerate_pixels() {
        let filter_center = (filter.len() / 2, filter.len() / 2);
        let x = pixel.0;
        let y = pixel.1;
        let mut sum: f64 = 0.0;
        // for j in 0..filter.len() {
        //     for i in 0..filter.len() {
        //         if x - (filter_center.0 - i) <= x {
        //             sum += image.get_pixel(x + filter_center.0 - i, y).data[0];
        //             sum += image.get_pixel(x - (filter.center.0 - i), y).data[0];
        //         }
        //         if y - (filter_center.1 - j) < y {
        //             sum += y;
        //         }
        //     }
        // }
        let begin_x: isize = x as isize - (filter.len() / 2) as isize;
        let begin_y: isize = y as isize - (filter.len() / 2) as isize;
        let filter_len = filter.len() as isize;
        println!("begin x: {}, begin y: {}", begin_x, begin_y);
        // TODO build a small image maybe 5 x 5 to test out filters and make it easy to keep track
        // of calculations
        for j in begin_y..filter_len-1 {
            for i in begin_x..filter_len-1 {
                if i < 0 && j < 0 {
                    println!("top left {}, {}, pixel: {}", i, j, image.get_pixel(x, y).data[0]);
                     sum += image.get_pixel(x, y).data[0] as f64;
                } else if j < 0 {
                    println!("top left {}, {}, pixel: {}", i, j, image.get_pixel(i as u32, y).data[0]);
                    sum += image.get_pixel(i as u32, y).data[0] as f64; 
                } else if i < 0 {
                    println!("top left {}, {}, pixel: {}", i, j, image.get_pixel(x, j as u32).data[0]);
                    sum += image.get_pixel(x, j as u32).data[0] as f64;
                } else if false {
                    
                } else {
                    println!("else block");
                    sum += image.get_pixel(i as u32, j as u32).data[0] as f64;
                }
                // println!("i: {}, j: {}", i, j);
                

            }
        }
        dbg!(sum);
        dbg!(sum / 9.0);
        dbg!((sum / 9.0).round());
        let average = (sum / 9.0).round() as u8;

        use image::Luma;
        let new_pixel = Luma([average]);
        new_image.put_pixel(x, y, new_pixel);
    }

    new_image
}

pub fn box_3x3_mut(image: &mut GrayImage) {
    let new_image = image.clone();

}
