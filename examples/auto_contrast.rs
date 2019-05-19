// extern crate image;
// extern crate image_processing;

// use image::ConvertBuffer;
use image_processing::color_space::hsv::*;
use image_processing::pixel_operations::auto_contrast_mut;
use image_processing::window::display_multiple_images;

pub fn main() {
    let img = image::open("images/england-hampton-court-palace.jpg")
        .expect("couldn't find image at that path")
        .to_rgba();
    // let base_img = img.clone();
    let mut hsv_image = HSV::from_image(&img);

    auto_contrast_mut(hsv_image.intensity_data_mut());
    let (width, height) = (500, 500);
    let new_image = hsv_image.to_rgb_image();

    display_multiple_images(
        &["original", "auto contrast"],
        &[&img, &new_image],
        width,
        height,
    );

    //     let img = image::open("images/empire-state-building-black-white-dark.jpg")
    //         .expect("couldn't find image at that path")
    //         .to_luma_alpha();

    //     let auto_contrast_img = auto_contrast(&img);
    //     let modified_auto_contrast_img = modified_auto_contrast(&img);
    //     let (width, height) = (500, 500);

    //     display_multiple_images(
    //         &["original", "auto contrast", "modified auto_contrast"],
    //         &[
    //             &img.convert(),
    //             &auto_contrast_img.convert(),
    //             &modified_auto_contrast_img.convert(),
    //         ],
    //         width,
    //         height,
    //     );
}
