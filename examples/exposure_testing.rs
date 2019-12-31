#[cfg(feature = "display-window")]
fn main() {
    use image::ConvertBuffer;
    use image::GrayImage;
    use image::ImageBuffer;
    use image::RgbaImage;
    use image_processing::conversion::*;
    use image_processing::exposure::*;
    use image_processing::matrix_ops::*;
    use image_processing::pixel_ops::power_law_transform_mut;
    use image_processing::window::*;
    use image_processing::clamp;

    // let mut image = image::open("./images/england-hampton-court-palace.jpg")
    // let mut image = image::open("./images/empire-state-building.jpg")
    let mut image = image::open("./images/test.jpg")
        .expect("image not found")
        .to_rgba();

    let (width, height) = (800, 800);

    let compensation = 1.0;
    let mut hsl = Hsl::from(&image);
    for value in hsl.luminance.iter_mut() {
        let new_value = *value * 2.0_f32.powf(compensation);
        *value = clamp(new_value, 0.0, 1.0);
    }
    let rgba = RgbaImage::from(&hsl);
    // power_law_transform_mut(&mut image, 2.2);
    // srgb_to_rgb(&mut image);
    // let mut comp_image = exposure_compensation(&image, 1.0);
    // rgb_to_srgb(&mut comp_image);
    // power_law_transform_mut(&mut comp_image, 1.0/2.2);

    // display_image("exposure_compensation", &rgba, width, height);
    // display_image("exposure_compensation", &comp_image, width, height);
    display_multiple_images(
        &["regular", "converted"],
        &[&image, &rgba],
        width,
        height,
    );
}

#[cfg(not(feature = "display-window"))]
fn main() {
    panic!("Displaying images is supported only when the feature 'display window' is enabled.");
}
