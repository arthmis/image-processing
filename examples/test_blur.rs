use image_processing::blur::*;
use image::ConvertBuffer;
use image_processing::matrix_ops::*;
use image::GrayImage;
use image::ImageBuffer;

#[cfg(feature = "display-window")]
fn main() {
    use image_processing::window::*;
    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();

    let (width, height) = (800, 800);

    let mut box_image = image.clone();

    let size = 11;

    box_filter_mut(MeanKernel::new(size), &mut box_image);

    display_image("box", &box_image.convert(), width, height);
}

#[cfg(not(feature = "display-window"))]
fn main() {
    panic!("Displaying images is supported only when the feature 'display window' is enabled.");
}