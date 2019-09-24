#[macro_use]
extern crate criterion;
extern crate image_processing;

use criterion::Criterion;
use criterion::black_box;
use image;
use image::imageops;
use imageproc::filter::gaussian_blur_f32;
use core::time::Duration;

use image_processing::blur::*;

pub fn blur(c: &mut Criterion) {
    let mut group = c.benchmark_group("Blur");
    group.confidence_level(0.05);
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(150));

    let mut gauss_image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();

    let mut box_image = gauss_image.clone();
    let mut box_image_1 = box_image.clone();
    let mut box_image_2 = box_image.clone();
    let image_blur = gauss_image.clone();
    let imageproc_blur = gauss_image.clone();

    let gauss = GaussianKernel::new(100);
    let size = 7;
    let box_filter = MeanKernel::new(size);

    // group.bench_function("fast box horizontal", |b| {
    //     b.iter(|| {
    //         fast_box_blur(black_box(box_filter), black_box(&mut box_image));
    //     });
    // });
    // group.bench_function("Gaussian", |b| {
    //     b.iter(|| gaussian_filter_mut(black_box(&gauss), black_box(&mut gauss_image)));
    // });
    // group.bench_function("separable box", |b| {
    //     b.iter(|| mean_filter_mut(black_box(box_filter), black_box(&mut box_image_1)));
    // });
    group.bench_function("faster box horizontal", |b| {
        b.iter(|| faster_box_blur(black_box(box_filter), black_box(&mut box_image_2)));
    });
    // group.bench_function("Image Gaussian", |b| {
    //     b.iter(|| imageops::blur(black_box(&image_blur), black_box(1.0)));
    // });
    // group.bench_function("ImageProc Gaussian", |b| {
    //     b.iter(|| gaussian_blur_f32(black_box(&imageproc_blur), black_box(3.0)));
    // });

    group.finish();
}

criterion_group!(benches, blur);
criterion_main!(benches);