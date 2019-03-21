use image;
use image_processing;

fn main() {

    let image = image::open("images/base.jpg").expect("shell not in correct directory").to_luma();
    let box_filter_image = image_processing::filters::box_3x3(&image);

    use image_processing::window;
    use image::ConvertBuffer;

    window::display_multiple_images(&["image", "box filter image"], &[&image.convert(), &box_filter_image.convert()], 500, 500);
}
