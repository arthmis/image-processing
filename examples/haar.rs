use image_processing;
use image_processing::face_detection::haar_features;
use image_processing::statistics::integral_image::*;
use image;

fn main() {
    let img = image::open("images/empire-state-building.jpg")
        .expect("couldn't read empire state building jpeg")
        .to_luma();
    let (integral_img, integral_img_variance) = integral_image(&img); 
    let difference = haar_features::two_rectangles_vertical(&integral_img); 
    println!("difference: {}", difference);
    println!("last point: {}", integral_img.get_point(integral_img.width() - 1, integral_img.height() - 1));

}