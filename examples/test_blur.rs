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

    // let mut box_image =
    //     ImageBuffer::from_vec(2, 1, vec![22, 31, 40, 255, 33, 80, 90, 255]).unwrap();
    // let mut box_naive = box_image.clone();
    // dbg!(&box_image);
    // let size = 5.0;
    // box_filter_mut(size, &mut box_image);
    // naive_box_filter_mut(size as u32, &mut box_naive);
    // dbg!(&box_image);
    // dbg!(&box_naive);

    let mut box_image = image.clone();
    let mut box_naive = image.clone();
    let mut gaussian_image = image.clone();
    let size = 4.0;
    box_blur_mut(BoxKernel::new(11), &mut box_image);
    let filter = GaussianKernel::new(3);
    // gaussian_filter_mut(&filter, &mut gaussian_image);
    // naive_box_filter_mut(size as u32, &mut box_naive);
    display_image("box", &box_image, width, height);
    // display_multiple_images(
    //     &["box", "gauss"],
    //     &[&box_image, &gaussian_image],
    //     width,
    //     height,
    // );
}

// #[cfg(not(feature = "display-window"))]
// fn main() {
//     panic!("Displaying images is supported only when the feature 'display window' is enabled.");
// }
