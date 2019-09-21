#[macro_use]
extern crate criterion;
extern crate image_processing;

use criterion::Criterion;
use criterion::black_box;
use image;
use core::time::Duration;

use image_processing::blur::*;

pub fn blur(c: &mut Criterion) {
    let mut group = c.benchmark_group("Blur");
    group.confidence_level(0.05);
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(200));

    let mut gauss_image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();

    let mut box_image = gauss_image.clone();

    let gauss = GaussianKernel::new(1);
    let box_filter = MeanKernel::new(5);

    group.bench_function("Gaussian", |b| {
        b.iter(|| gaussian_filter_mut(black_box(gauss.clone()), black_box(&mut gauss_image)));
    });
    group.bench_function("Box", |b| {
        b.iter(|| mean_filter_mut(black_box(box_filter.clone()), black_box(&mut box_image)));
    });

    group.finish();
}

criterion_group!(benches, blur);
criterion_main!(benches);