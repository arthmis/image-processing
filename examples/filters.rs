use image::{self, ImageBuffer, GrayImage};
use image_processing::filters::box_3x3;

fn main() {

    let image = image::open("images/base.jpg").expect("shell not in correct directory").to_luma();
    let raw_buffer = vec![
        5,  6,  7,
        83, 23, 53,
        8,  9,  10,
    ];
    // let image: GrayImage = ImageBuffer::from_raw(3, 3, raw_buffer).unwrap();
    let box_filter_image = box_3x3(&image, 99, 99);

    use image_processing::window;
    use image::ConvertBuffer;
    let (width, height) = (1000, 1000);

    window::display_multiple_images(&["image", "box filter image"], &[&image.convert(), &box_filter_image.convert()], width, height);
}
