pub fn transpose(buf: &[u8], out_buf: &mut [u8], width: usize, height: usize)  {
    assert!(
        width * height == buf.len(), 
        "width * height not equal buf.len(): {} {}", 
        width * height, 
        buf.len()
    );
    assert!(
        buf.len() == out_buf.len(),
        "buf length: {}, out_buf length: {}", 
        buf.len(), 
        out_buf.len()
    );

    for y in 0..height {
        for x in 0..width {
            out_buf[x * height + y] = buf[y * width + x]; 
        }
    }
}

pub fn fast_transpose(buf: &[u8], out_buf: &mut [u8], width: usize, height: usize)  {
    assert!(
        width * height == buf.len(), 
        "width * height not equal buf.len(): {} {}", 
        width * height, 
        buf.len()
    );
    assert!(
        buf.len() == out_buf.len(),
        "buf length: {}, out_buf length: {}", 
        buf.len(), 
        out_buf.len()
    );

    let block: usize = 16;
    for y in (0..height).step_by(block) {
        for x in (0..width).step_by(block) {
            for k in (y..y + block) {
                if k >= height {
                    continue;
                }
                for j in (x..x + block) {
                    if j >= width {
                        continue;
                    } 
                    // out_buf[j * height + k] = buf[k * width + j]; 
                    unsafe {
                        let out_elem = out_buf.get_unchecked_mut(j * height + k);
                        *out_elem = *buf.get_unchecked(k * width + j);
                    }
                }
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
    fn test_fast_transpose() {
        let mut matrix: [u8; 9] = [0, 1, 2, 3, 4, 5, 6, 7, 8];
        let mut matrix_transpose: [u8; 9] = [0; 9];

        fast_transpose(&matrix, &mut matrix_transpose, 3, 3);
        for y in 0..3 {
            for x in 0..3 {
                let first_val = matrix[y * 3 + x];
                let second_val = matrix_transpose[x * 3 + y];
                assert!(first_val == second_val, "{} {}", first_val, second_val); 
            }
        }
    }
}