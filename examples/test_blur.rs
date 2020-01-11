#[cfg(feature = "display-window")]
fn main() {
    use image::ConvertBuffer;
    use image::GrayImage;
    use image::ImageBuffer;
    use image::RgbaImage;
    use image_processing::blur::*;
    use image_processing::matrix_ops::*;
    use image_processing::window::*;

    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_rgba();
    // .to_luma();

    let (width, height) = (800, 800);

    let mut box_image = image.clone();
    let raw_image: Vec<u8> = vec![
        100, 125, 150, 255, 25, 50, 75, 255, 200, 225, 250, 255, 100, 125, 150, 255, 25, 50, 75,
        255, 200, 225, 250, 255,
    ];
    let mut image: RgbaImage = ImageBuffer::from_vec(3, 2, raw_image).unwrap();

    let size = 11;

    box_filter_mut(MeanKernel::new(size), &mut box_image);
    // box_filter_mut_alternate(MeanKernel::new(size), &mut box_image);

    // display_image("box", &box_image, width, height);
    display_image("box", &box_image, width, height);
}

#[cfg(not(feature = "display-window"))]
fn main() {
    panic!("Displaying images is supported only when the feature 'display window' is enabled.");
}
