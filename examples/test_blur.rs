#[cfg(feature = "display-window")]
fn main() {
    use image::GrayImage;
    use image::ImageBuffer;
    use image::RgbaImage;
    use image_processing::blur::*;
    use image_processing::matrix_ops::*;
    use image_processing::window::*;

    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_rgba();

    let (width, height) = (800, 800);

    let mut box_image = image.clone();
    let size = 15;

    box_image = box_filter_mut(MeanKernel::new(size), box_image);

    display_image("box", &box_image, width, height);
}

#[cfg(not(feature = "display-window"))]
fn main() {
    panic!("Displaying images is supported only when the feature 'display window' is enabled.");
}
