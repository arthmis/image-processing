use image::RgbaImage;
use image::{GenericImage, GenericImageView};
// pub fn transpose(buf: &[u8], transpose: &mut [u8], width: usize, height: usize)  {
pub fn transpose(image: &RgbaImage, transpose: &mut RgbaImage)  {

    // assert!(
    //     width * height * stride == buf.len(), 
    //     "width * height not equal buf.len(): {} {}", 
    //     width * height, 
    //     buf.len()
    // );
    // assert!(
    //     buf.len() == transpose.len(),
    //     "buf length: {}, out_buf length: {}", 
    //     buf.len(), 
    //     transpose.len()
    // );

    let block: usize = 16;
    // for (chunk, transpose_chunk) in buf.chunks(block).zip(transpose.chunks(block)) {
    //     for elem in chunk.iter().zip(transpose_chunk.iter()) {

    //     }
    // }
    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let transpose_pixel = transpose.get_pixel_mut(y, x);
            *transpose_pixel = *pixel;
        }
    }
    // for (pixel, transpose_pixel) in image.pixels().zip(transpose.pixels_mut()) {


    // }
  
}
const stride: usize = 4;
fn index(coordinates: (usize, usize), width: usize) -> usize {
    coordinates.1 * width * stride + coordinates.0 * stride
}

pub fn faster_transpose(buf: &[u8], transpose: &mut [u8], width: usize, height: usize)  {
    assert!(
        width * height == buf.len(), 
        "width * height not equal buf.len(): {} {}", 
        width * height, 
        buf.len()
    );
    assert!(
        buf.len() == transpose.len(),
        "buf length: {}, out_buf length: {}", 
        buf.len(), 
        transpose.len()
    );

    let block: usize = 16;
    let x_end = width / block;
    let y_end = height / block;

    for y in (0..y_end).step_by(block) {
        for x in (0..x_end).step_by(block) {
            for k in y..y + block {
                for j in x..x + block {
                    unsafe {
                        let out_elem = transpose.get_unchecked_mut(j * height + k);
                        *out_elem = *buf.get_unchecked(k * width + j);
                    }
                }
            }
        }
    }
    for y in y_end..height {
        for x in x_end..width {
            unsafe {
                let out_elem = transpose.get_unchecked_mut(x * height + y);
                *out_elem = *buf.get_unchecked(y * width + x);
            }

        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_transpose() {
        let mut matrix: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut matrix_transpose: [u8; 9] = [0; 9];

        transpose(&matrix, &mut matrix_transpose, 3, 3);
        for y in 0..3 {
            for x in 0..3 {
                let first_val = matrix[y * 3 + x];
                let second_val = matrix_transpose[x * 3 + y];
                assert!(first_val == second_val, "{} {}", first_val, second_val); 
            }
        }
    }

    #[test]
    fn test_transpose_on_color_image() {
        let matrix: [u8; 16] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];
        let mut matrix_transpose: [u8; 16] = [0; 16];
        let (width, height) = (2, 2);
        transpose(&matrix, &mut matrix_transpose, width, height);
        for y in 0..height {
            for x in 0..width {
                let first_val = matrix[y * width + x];
                let second_val = matrix_transpose[x * height + y];
                assert!(first_val == second_val, "{} {}", first_val, second_val); 

            }
        }
    }

    #[test]
    fn test_faster_transpose() {
        let mut matrix: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut matrix_transpose: [u8; 9] = [0; 9];

        faster_transpose(&matrix, &mut matrix_transpose, 3, 3);
        for y in 0..3 {
            for x in 0..3 {
                let first_val = matrix[y * 3 + x];
                let second_val = matrix_transpose[x * 3 + y];
                assert!(first_val == second_val, "{} {}", first_val, second_val); 
            }
        }
    }
}