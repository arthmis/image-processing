extern crate image;
extern crate image_processing;

use image_processing::pixel_operations::*;
use image_processing::statistics::histogram::*;
use image_processing::window::display_multiple_images;

use image::ConvertBuffer;

fn main() {
    let img = image::open("images/england-hampton-court-palace.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();
    let next_img = image::open("images/montauk-lighthouse.jpg")
        .expect("couldn't find image at that path")
        .to_luma_alpha();
    let width = 500;
    let height = 500;
    // let inverted_img = invert_grayscale(&img);
    // let thresholded_image = threshold(&img, 125);
    // let contrast_image = contrast(&img, 50);
    // let normalized_image = equalize_histogram(&img);
    // let normalized_hist = graya_histogram(&normalized_image);
    // let eq_image_hist_img = convert_to_image(width, height, &eq_image_hist);
    // let inverted_img_hist = graya_histogram(&inverted_img);
    // let thresholded_image_hist = graya_histogram(&thresholded_image);
    // let contrast_hist = graya_histogram(&contrast_image);
    // let base_hist = graya_histogram(&img);
    let histogram_specification = match_piecewise_linear_histogram(&img, &next_img);
    // let target_histogram = graya_histogram(&next_img);
    // let target_histogram_image = convert_to_image(width, height, &target_histogram);
    // let histogram_specification_histogram = graya_histogram(&histogram_specification);
    // let histogram_specification_histogram = convert_to_image(width, height, &histogram_specification_histogram);

    display_multiple_images(
        &[
            "base",
            "histogram specification",
            "target image",
            // "target image histogram",
            // "histogram specification histogram"
    //         // "inverted",
    //         // "thresholded",
    //         "normalized image",
    //         "base histogram",
    //         "normalized histogram",
    //         // "inverted histogram",
    //         // "thresholded histogram",
    //         // "contrast histogram",
        ],
        &[
            &img.convert(),
            &histogram_specification.convert(),
            &next_img.convert(),
            // &target_histogram_image.convert(),
            // &histogram_specification_histogram.convert()
    //         // &inverted_img.convert(),
    //         // &thresholded_image.convert(),
    //         &normalized_image.convert(),
    //         &convert_to_image(width, height, &base_hist).convert(),
    //         &convert_to_image(width, height, &normalized_hist).convert(),
    //         // &convert_to_image(width, height, &inverted_img_hist).convert(),
    //         // &convert_to_image(width, height, &thresholded_image_hist).convert(),
    //         // &convert_to_image(width, height, &contrast_hist).convert(),
        ],
        width,
        height,
    );
    img.save("images/base.jpg").unwrap();
    histogram_specification.save("images/matched_histogram.jpg").unwrap();
    next_img.save("images/reference_image.jpg").unwrap();
}
