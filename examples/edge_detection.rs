
#[cfg(feature = "display-window")]
fn main() {
    use image_processing::window::*;
    use image_processing::edge_detection::*;
    use image::ConvertBuffer;
    use image_processing::pixel_ops::threshold_mut;

    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();

    let mut image_clone = image.clone();
    let (width, height) = (800, 800);
    // let mut x_image = image.clone();
    // let mut y_image = image.clone();

    // sobel_x(&mut x_image);
    // sobel_y(&mut y_image);

    let threshold = 120;
    sobel_mut(&mut image, threshold);
    slower_sobel_mut(&mut image_clone, threshold);
    // threshold_mut(&mut x_image, threshold);
    // threshold_mut(&mut y_image, threshold);

    // display_image("sobel horizontal", &image.convert(), width, height);
    display_multiple_images(
        &["sobel regular", "sobel fast"], 
        &[&image.convert(), &image_clone.convert()], 
        width, 
        height
    );
    // display_multiple_images( 
    //     &["sobel", "sobel x ", "sobel y"], 
    //     &[&image.convert(), &x_image.convert(), &y_image.convert()], 
    //     width, 
    //     height
    // );

}

#[cfg(not(feature = "display-window"))]
fn main() {
    panic!("Displaying images is supported only when the feature 'display window' is enabled.");
}