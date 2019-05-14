use image_processing::color_space::hsv::*;

fn main() {
    let mut hsv = HSV::new(20, 20);
    for hue in hsv.hues() {
        println!("{}", hue);
    }
}