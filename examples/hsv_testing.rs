use image_processing::color_space::hsv::*;
use image::RgbaImage;
use image;
use image_processing::window::display_multiple_images;
// use image::ConvertBuffer;

fn main() {

    let image = image::open("images/england-hampton-court-palace.jpg")
        .expect("couldn't find image at that path")
        .to_rgba();

    let hsv_image = HSV::from_image(&image);
    let convert_image = hsv_image.to_rgb_image();
    let (width, height) = (1000, 1000);

    display_multiple_images(
        &["original", "converted back"],
        &[&image, &convert_image],
        width, 
        height,
    );

    let raw_data = vec![
        25, 33, 44, 255,
        88, 21, 30, 255,
        99, 0, 63, 255,
        156, 200, 201, 255,
    ];
    let image: RgbaImage = image::ImageBuffer::from_raw(2, 2, raw_data).unwrap(); 
    let hsv_image = HSV::from_image(&image);
    let convert_image = hsv_image.to_rgb_image();
    for pixel in convert_image.pixels() {
        // println!("{} {} {}", pixel[0], pixel[1], pixel[2]);
    }
    // println!();
    // for ((hue, saturation), brightness) in hsv_image.hues()
    //     .zip(hsv_image.saturations())
    //     .zip(hsv_image.intensities()) 
    // {
    //     // println!("hue: {}, saturation: {}, brightness: {}", hue, saturation, brightness);
    // }

    let red: f32 = 0.098;
    let green: f32 = 0.1294;
    let blue: f32 = 0.1725;
}