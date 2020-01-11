use image::ConvertBuffer;
use image::GrayImage;
use image::ImageBuffer;
use image_processing::pixel_ops::*;

#[cfg(feature = "display-window")]
fn main() {
    use image_processing::window::*;
    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();
    let original = image.clone();

    let (width, height) = (800, 800);
    // logarithm_mut(&mut image);
    // power_law_transform_mut(&mut image, 0.6);
    lut_power_law_transform_mut(&mut image, 2.2);

    // display_image("naive sharpening", &image.convert(), width, height);
    display_multiple_images(
        // &["original", "logarithm"],
        &["original", "exponential"],
        &[&original.convert(), &image.convert()],
        width,
        height,
    );
}

#[cfg(not(feature = "display-window"))]
fn main() {
    panic!("Displaying images is supported only when the feature 'display window' is enabled.");
}
