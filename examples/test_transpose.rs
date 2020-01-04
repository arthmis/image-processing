use image_processing::matrix_ops::*;
use image::{RgbaImage, GrayImage};
use image::ConvertBuffer;
use image::ImageBuffer;

fn main() {

    // let mut matrix  = vec![
    //     0, 1, 2, 255, 3, 4, 5, 255, 6, 7, 8, 255, 
    //     11, 12, 13, 255, 14, 15, 16, 255, 17, 18, 19, 255
    // ];
    // let mut matrix_transpose = vec![0_u8; 24];

    // let (width, height) = (3_usize, 2_usize);
    // let stride: usize = 4;
    // transpose_generic(&matrix, &mut matrix_transpose, width, height, stride);
    // // for y in 0..height {
    // //     for x in 0..width {
    // //         // let thing: &[u8] = &matrix[(y * width * stride + x)..(x * stride + stride)];
    // //         // print!("{:?}", thing);
    // //         // for i in (x * stride)..(x * stride + stride)  {
    // //         //     // print!("{} ", matrix[y * width * stride + i]);
    // //         //     // let val = matrix[y * width * stride + i];
    // //         //     println!("{}", x * height * stride + i);
    // //         //     // matrix_transpose[x * height * stride + i] = val;
    // //         // }
    // //         let min_index = (y * width + x) * stride;
    // //         let transpose_min_index = (x * height + y) * stride;
    // //         let matrix_val: &[u8] = &matrix[min_index..(min_index + stride)];
    // //         let transpose_val: &mut [u8] = &mut matrix_transpose[transpose_min_index..(transpose_min_index + stride)];
    // //         // print!("{} ", transpose_min_index);
    // //         // *transpose_val = *matrix_val;
    // //         for (orig, trans) in matrix_val.iter().zip(transpose_val.iter_mut()) {
    // //             *trans = *orig;
    // //         }
    // //         // print!("{:?}", &matrix[min_index..(min_index + stride)]);
    // //     }
    // // }
    // println!();
    // println!("{:?}", matrix_transpose);
    // // transpose(&matrix, &mut matrix_transpose, width, height);
    let image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_rgba();
    let (width, height) = image.dimensions();

    let gray_image: GrayImage = image.convert();
    let gray_image_copy = gray_image.clone();
    let mut third_buffer: GrayImage = ImageBuffer::new(height, width);
    matrix_transpose(
        &gray_image, 
        &mut third_buffer,
        width as usize, 
        height as usize,
    );

    let mut fourth_buffer: GrayImage = ImageBuffer::new(height, width);
    transpose_generic(
        &gray_image_copy, 
        &mut fourth_buffer,
        width as usize, 
        height as usize,
        1, 
    );


}