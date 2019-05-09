// use image::GrayImage;

// /// If the filter width or height are not odd in size then they are made into odd lengths
// /// by subtracting 1
// /// If the width or height is less than 3 then the input image is returned without any convolution done
// pub fn box_3x3(image: &GrayImage, filter_width: u32, filter_height: u32) -> GrayImage {
//     let mut new_image = image.clone();

//     let (image_width, image_height) = image.dimensions();

//     let filter_width: u32 = if filter_width % 2 == 0 {
//         filter_width - 1
//     } else {
//         filter_width
//     };

//     let filter_height: u32 = if filter_height % 2 == 0 {
//         filter_height - 1
//     } else {
//         filter_height
//     };

//     if filter_width < 3 || filter_height < 3 {
//         return new_image;
//     }

//     for pixel in image.enumerate_pixels() {
//         let x = pixel.0;
//         let y = pixel.1;
//         let mut sum: u32 = 0;

//         let begin_x: isize = x as isize - (filter_width / 2) as isize;
//         let begin_y: isize = y as isize - (filter_height / 2) as isize;
//         let end_x = begin_x + filter_width as isize;
//         let end_y = begin_y + filter_height as isize;

//         for j in begin_y..end_y {
//             for i in begin_x..end_x {
//                 if i < 0 && j < 0 {
//                     // top left
//                     sum += image.get_pixel(0, 0).data[0] as u32;
//                 } else if i > (image_width - 1) as isize && j < 0 {
//                     // top right
//                     sum += image.get_pixel(image_width - 1, 0).data[0] as u32;
//                 } else if j < 0 {
//                     // top
//                     sum += image.get_pixel(i as u32, 0).data[0] as u32;
//                 } else if i < 0 && j > (image_height as isize) - 1 {
//                     // bottom left
//                     sum += image.get_pixel(0, image_height - 1).data[0] as u32;
//                 } else if i > image_width as isize - 1 && j > image_height as isize - 1 {
//                     // bottom right
//                     sum += image.get_pixel(image_width - 1, image_height - 1).data[0] as u32;
//                 } else if i < 0 {
//                     // left
//                     sum += image.get_pixel(0, j as u32).data[0] as u32;
//                 } else if i > image_width as isize - 1 {
//                     // right
//                     sum += image.get_pixel(image_width - 1, j as u32).data[0] as u32;
//                 } else if j > image_height as isize - 1 {
//                     // bottom
//                     sum += image.get_pixel(i as u32, image_height - 1).data[0] as u32;
//                 } else {
//                     sum += image.get_pixel(i as u32, j as u32).data[0] as u32;
//                 }
//             }
//         }

//         let average = (sum as f64 / (filter_height * filter_width) as f64).round() as u8;
//         use image::Luma;
//         let new_pixel = Luma([average]);
//         new_image.put_pixel(x, y, new_pixel);
//     }

//     new_image
// }

// // pub fn box_3x3_mut(image: &mut GrayImage) {
// //     let new_image = image.clone();
// // }
