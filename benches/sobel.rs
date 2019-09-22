#[macro_use]
extern crate criterion;
extern crate image_processing;

use criterion::Criterion;
use criterion::black_box;
use image;
use core::time::Duration;

use image_processing::edge_detection::*;

pub fn sobel_x_y(c: &mut Criterion) {
    let mut group = c.benchmark_group("Sobel Edge Detector");
    group.confidence_level(0.01);
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(200));

    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();

    let mut x_image = image.clone();
    let mut y_image = image.clone();

    group.bench_function("Sobel full", move |b| {
        b.iter(|| sobel_mut(black_box(&mut image)));
    });
    group.bench_function("Sobel x", move |b| {
        b.iter(|| sobel_x(black_box(&mut x_image)));
    });
    group.bench_function("Sobel y", move |b| {
        b.iter(|| sobel_y(black_box(&mut y_image)));
    });

    group.finish();
}

criterion_group!(benches, sobel_x_y);
criterion_main!(benches);