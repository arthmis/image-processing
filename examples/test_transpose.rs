use image::ConvertBuffer;
use image::ImageBuffer;
use image::{GrayImage, RgbaImage};
use image_processing::matrix_ops::*;

fn main() {
    let mut matrix = vec![
        0, 1, 2, 255, 3, 4, 5, 255, 6, 7, 8, 255, 11, 12, 13, 255, 14, 15, 16, 255, 17, 18, 19, 255,
    ];
    let mut matrix_transpose = vec![0_u8; 24];

    let (width, height) = (3_usize, 2_usize);
    let stride: usize = 4;
    // let image = image::open("./images/england-hampton-court-palace.jpg")
    //     .expect("image not found")
    //     .to_rgba();
    // let (width, height) = image.dimensions();

    // let gray_image: GrayImage = image.convert();
    // let gray_image_copy = gray_image.clone();
    // let mut third_buffer: GrayImage = ImageBuffer::new(height, width);
    // matrix_transpose(
    //     &gray_image,
    //     &mut third_buffer,
    //     width as usize,
    //     height as usize,
    // );

    let mut fourth_buffer: RgbaImage = ImageBuffer::new(height as u32, width as u32);
    transpose_generic(
        &matrix_transpose,
        &mut fourth_buffer,
        width as usize,
        height as usize,
        stride,
    );
    println!("{:?}", fourth_buffer);
}
