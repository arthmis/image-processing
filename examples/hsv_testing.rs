use image_processing::color_space::hsv::*;

fn main() {
    let mut hsv = HSV::new(20, 20);
    hsv.hue[0] = 1;
    for hue in hsv.hues() {
        println!("{}", hue);
    }
}