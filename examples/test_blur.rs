#[cfg(feature = "display-window")]
fn main() {
    // use image::ConvertBuffer;
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
    // let raw_image: Vec<u8> = vec![
    //     100, 125, 150, 255, 25, 50, 75, 255, 200, 225, 250, 255, 100, 125, 150, 255, 25, 50, 75,
    //     255, 200, 225, 250, 255,
    // ];
    // let mut image: RgbaImage = ImageBuffer::from_vec(3, 2, raw_image).unwrap();

    let size = 11;

    box_image = box_filter_mut(MeanKernel::new(size), box_image);
    naive_box_filter_mut(MeanKernel::new(size), &mut image);
    // box_filter_mut_alternate(MeanKernel::new(size), &mut box_image);

    display_image("box", &box_image, width, height);
    // display_image("box", &image, width, height);
    // use std::io::Write;
    // let stdout = std::io::stdout();
    // let mut handle = stdout.lock();
    // for (naive, fast) in image.pixels().zip(box_image.pixels()) {
    //     if naive[0] != fast[0] {
    //         println!("red: {} {}", naive[0], fast[0])
    //     }
    //     if naive[1] != fast[1] {
    //         println!("green: {} {}", naive[1], fast[1])
    //     }
    //     if naive[2] != fast[2] {
    //         println!("blue: {} {}", naive[2], fast[2])
    //     }
    //     println!();
    //     // assert_eq!(naive[0], fast[0]);
    //     // assert_eq!(naive[1], fast[1]);
    //     // assert_eq!(naive[2], fast[2]);
    // }
}

#[cfg(not(feature = "display-window"))]
fn main() {
    panic!("Displaying images is supported only when the feature 'display window' is enabled.");
}
