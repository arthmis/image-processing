extern crate image;
extern crate image_processing;

use image_processing::pixel_operations::*;
// use image_processing::statistics::histogram::*;
// use image::RgbaImage;
use image_processing::color_space::hsv::*;
use image_processing::window::display_multiple_images;

// use image::ConvertBuffer;

fn main() {
    let mut img = image::open("images/england-hampton-court-palace.jpg")
        .expect("couldn't find image at that path")
        .to_rgba();
    let base_img = img.clone();
    let mut hsv_image = HSV::from_image(&img);
    // let original_intensities = hsv_image.intensity_data.clone();

    exposure_compensation_mut(hsv_image.intensity_data_mut(), 1.0);
    exposure_compensation_mut_rgb(&mut img, 1.0);
    let (width, height) = (500, 500);
    let new_image = hsv_image.to_rgb_image();
    let after_EC = HSV::from_image(&new_image); 

    display_multiple_images(
        &["original", "hsv compensated", "rgb compensated" ],
        &[&base_img, &new_image, &img],
        width,
        height,
    );

    // change_saturation_mut(&mut hsv_image.saturation_data_mut(), 1.0);
    // let new_image = hsv_image.to_rgb_image();
    // change_saturation_mut(&mut hsv_image.saturation_data_mut(), 1.0);
    // let img = hsv_image.to_rgb_image();
    // display_multiple_images(
    //     &["original", "saturation change", "saturation change 2 times"],
    //     &[&base_img, &new_image, &img],
    //     width,
    //     height,
    // );

    // new_img.save("exposure_compensation.jpg").unwrap();

    // for ((new_pixel, old_pixel), rgb_new_pixel) in new_image.pixels().zip(base_img.pixels()).zip(img.pixels()) {
    //     println!("red: {} green: {} blue: {}", old_pixel[0], old_pixel[1], old_pixel[2]);
    //     println!("hsv -> red: {} green: {} blue: {}", new_pixel[0], new_pixel[1], new_pixel[2]);
    //     println!("rgb -> red: {} green: {} blue: {}", rgb_new_pixel[0], rgb_new_pixel[1], rgb_new_pixel[2]);
    //     println!();
    // }
    // for ((hue, saturation), brightness) in hsv_image.hues()

    //     .zip(hsv_image.saturations())
    //     .zip(hsv_image.intensities())
    // {
    //     println!("hue: {}, saturation: {}, brightness: {}", hue, saturation, brightness);
    // }
    // for ((hue, saturation), brightness) in hsv_image.hues()
    //     .zip(hsv_image.saturations())
    //     .zip(hsv_image.intensities())
    // {
    // for (new_intensity, original_intensity) in hsv_image.intensities().zip(original_intensities.iter()) {
    //     println!("original intensity: {}\nnew intensity: {}\n", *original_intensity, new_intensity);
    // }

    // let raw_data = vec![
    //     25, 33, 44, 255,
    //     88, 21, 30, 255,
    //     99, 0, 63, 255,
    //     156, 200, 201, 255,
    // ];
    // let image: RgbaImage = image::ImageBuffer::from_raw(2, 2, raw_data).unwrap();
    // let mut hsv_image = HSV::from_image(&image);
    // for ((hue, saturation), brightness) in hsv_image.hues()
    //     .zip(hsv_image.saturations())
    //     .zip(hsv_image.intensities())
    // {
    //     println!("hue: {}, saturation: {}, brightness: {}", hue, saturation, brightness);
    // }
    // exposure_compensation_mut(hsv_image.intensity_data_mut(), -1.0);
    // println!();
    // let convert_image = hsv_image.to_rgb_image();
    // for pixel in convert_image.pixels() {
    //     println!("red: {} green: {} blue: {}", pixel[0], pixel[1], pixel[2]);
    // }
    // println!();
    // for ((hue, saturation), brightness) in hsv_image.hues()
    //     .zip(hsv_image.saturations())
    //     .zip(hsv_image.intensities())
    // {
    //     println!("hue: {}, saturation: {}, brightness: {}", hue, saturation, brightness);
    // }
}
