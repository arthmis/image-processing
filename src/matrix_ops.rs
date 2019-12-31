use image::RgbaImage;
use image::{GenericImage, GenericImageView};
// pub fn gray_image_transpose(buf: &[u8], transpose: &mut [u8], width: usize, height: usize)  {
pub fn transpose(image: &RgbaImage, transpose: &mut RgbaImage)  {

    let (width, height) = image.dimensions();
    for y in 0..height {
        for x in 0..width {
            let pixel = image.get_pixel(x, y);
            let transpose_pixel = transpose.get_pixel_mut(y, x);
            *transpose_pixel = *pixel;
        }
    }
  
}
pub fn matrix_transpose(buf: &[u8], transpose: &mut [u8], width: usize, height: usize)  {
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

    for y in 0..height {
        for x in 0..width {
            transpose[x * height + y] = buf[y * width + x]; 
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_matrix_transpose() {
        let mut matrix: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut output_matrix: [u8; 9] = [0; 9];

        matrix_transpose(&matrix, &mut output_matrix, 3, 3);
        for y in 0..3 {
            for x in 0..3 {
                let first_val = matrix[y * 3 + x];
                let second_val = output_matrix[x * 3 + y];
                assert!(first_val == second_val, "{} {}", first_val, second_val); 
            }
        }
    }
}