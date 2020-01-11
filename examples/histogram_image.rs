#[cfg(feature = "display-window")]
fn main() {
    use image_processing::histogram::*;
    use image_processing::window::*;

    let image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_rgba();
    let histogram = LumaHistogram::from_rgba_image(&image);
    let hist_image = convert_to_image(256, 256, &histogram);
    let (width, height) = (800, 800);
    display_image("histogram image", &hist_image, width, height);
}

#[cfg(not(feature = "display-window"))]
fn main() {
    panic!("Displaying images is supported only when the feature 'display window' is enabled.");
}
