use image::GrayImage;

pub fn box_3x3(image: &GrayImage) -> GrayImage {
    let mut new_image = image.clone();
    let filter = [[1, 1, 1], [1, 1, 1], [1, 1, 1]];

    let image_height = image.height();
    let image_width = image.width();

    for pixel in image.enumerate_pixels() {
        let filter_center = (filter.len() / 2, filter.len() / 2);
        let x = pixel.0;
        let y = pixel.1;
        let mut sum: f64 = 0.0;

        let begin_x: isize = x as isize - (filter.len() / 2) as isize;
        let begin_y: isize = y as isize - (filter.len() / 2) as isize;
        let end_x = begin_x + 3;
        let end_y = begin_y + 3;

        let filter_len = filter.len() as isize;
        println!("\nnext");
        println!("begin x: {}, begin y: {}", begin_x, begin_y);
        println!("end   x: {}, end   y: {}", end_x, end_y);

        for j in begin_y..end_y {
            for i in begin_x..end_x {
                println!("i: {}, j: {}", i, j);
                if i < 0 && j < 0 { // top left
                    println!(
                        "top left        {}, {}, pixel: {}",
                        i,
                        j,
                        image.get_pixel(0, 0).data[0]
                    );
                    sum += image.get_pixel(0, 0).data[0] as f64;
                } else if j < 0 { // top
                    println!(
                        "top             {}, {}, pixel: {}",
                        i,
                        j,
                        image.get_pixel(i as u32, 0).data[0]
                    );
                    sum += image.get_pixel(i as u32, 0).data[0] as f64;
                } else if i < 0 { // left
                    println!(
                        "left           {}, {}, pixel: {}",
                        i,
                        j,
                        image.get_pixel(0, j as u32).data[0]
                    );
                    sum += image.get_pixel(0, j as u32).data[0] as f64;
                } else if i > image_width as isize - 1 && j < 0 { // top right
                    println!(
                        "left           {}, {}, pixel: {}",
                        i,
                        j,
                        image.get_pixel(image_width - 1, 0).data[0]
                    );
                    sum += image.get_pixel(image_width - 1, 0).data[0] as f64;
                } else if i > image_width as isize - 1 { // right
                    println!(
                        "left           {}, {}, pixel: {}",
                        i,
                        j,
                        image.get_pixel(image_width - 1, j as u32).data[0]
                    );
                    sum += image.get_pixel(image_width - 1, j as u32).data[0] as f64;
                } else if i < 0 && j > image_height as isize - 1 { // bottom left
                    sum += image.get_pixel(0, image_height - 1).data[0] as f64;
                } else if j > image_height as isize - 1 { // bottom
                    sum += image.get_pixel(i as u32, image_height - 1).data[0] as f64;
                } else if i > image_width as isize - 1 && j > image_height as isize - 1 { // bottom right
                    sum += image.get_pixel(image_width - 1, image_height - 1).data[0] as f64;
                } else {
                    println!("in image bounds {}, {}, pixel: {}", i, j, image.get_pixel(i as u32, j as u32).data[0]);
                    sum += image.get_pixel(i as u32, j as u32).data[0] as f64;
                }
                // println!("i: {}, j: {}", i, j);
            }
        }
        // dbg!(sum);
        // dbg!(sum / 9.0);
        // dbg!((sum / 9.0).round());
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
