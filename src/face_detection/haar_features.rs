use crate::drawing::rectangle::{Rectangle, Region};
use crate::statistics::integral_image::IntegralImage;

pub fn two_rectangles_vertical(integral_image: &IntegralImage) -> u64 {
    let region = Region::new(24, 24);
    let top_region: i64 = integral_image.region_sum(Rectangle::new(1, 24, 1, 12)) as i64;
    let bottom_region: i64 = integral_image.region_sum(Rectangle::new(1, 24, 12, 24)) as i64;
    let difference: u64 = (top_region - bottom_region).abs() as u64;
    println!(
        "top region: {}, bottom region: {}",
        top_region, bottom_region
    );

    difference
}
