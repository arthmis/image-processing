use image_processing::matrix_ops::*;

fn main() {

    let mut matrix: [u8; 12] = [0, 1, 2, 3, 4, 5, 6, 7, 8, 7, 10, 111];
    let mut matrix_transpose: [u8; 12] = [0; 12];
    let mut re_transpose: [u8; 12] = [0; 12];

    let (width, height) = (4, 3);
    // transpose(&matrix, &mut matrix_transpose, 4, 3);
    fast_transpose(&matrix, &mut matrix_transpose, width, height);
    fast_transpose(&matrix_transpose, &mut matrix, height, width);
    // dbg!(matrix_transpose);
    // dbg!(matrix);
    for y in 0..height {
        for x in 0..width {
            let first_val = matrix[y * width + x];
            let second_val = matrix_transpose[x * height + y];
            // dbg!(first_val);
            assert!(first_val == second_val, "{} {}", first_val, second_val); 
        }
    }
    for y in 0..height {
        println!();
        for x in 0..width {
            let val = matrix[y * width + x];
            // dbg!(val);
            print!("{} ", val);
        }
    }
    println!();
    for y in 0..width {
        println!();
        for x in 0..height {
            let val = matrix_transpose[y * height + x];
            // dbg!(val);
            print!("{} ", val);
        }
    }
    println!();
    fast_transpose(&matrix_transpose, &mut re_transpose, height, width);
    for y in 0..height {
        println!();
        for x in 0..width {
            let val = matrix[y * width + x];
            // dbg!(val);
            print!("{} ", val);
        }
    }

}