#[macro_use]
extern crate criterion;
extern crate image_processing;

use criterion::Criterion;
use criterion::black_box;
use image;
use core::time::Duration;

use image_processing::matrix_ops::*;
use image::{ImageBuffer, GrayImage};

pub fn blur(c: &mut Criterion) {
    let mut group = c.benchmark_group("Transpose");
    group.confidence_level(0.05);
    group.sample_size(50);
    group.measurement_time(Duration::from_secs(150));

    let mut image = image::open("./images/england-hampton-court-palace.jpg")
        .expect("image not found")
        .to_luma();
    let (width, height) = image.dimensions();

    let mut first_buffer: GrayImage = ImageBuffer::new(height, width);

    let mut second_image = image.clone();
    let mut second_buffer: GrayImage = ImageBuffer::new(height, width);

    group.bench_function("naive transpose", |b| {
        b.iter(|| {
            transpose(
                black_box(&image), 
                black_box(&mut first_buffer),
                black_box(width as usize), 
                black_box(height as usize), 
            );
        });
    });
    group.bench_function("manual cache blocked transpose", |b| {
        b.iter(|| {
            fast_transpose(
                black_box(&second_image), 
                black_box(&mut second_buffer),
                black_box(width as usize), 
                black_box(height as usize), 
            );
        });
    });

    group.finish();
}

criterion_group!(benches, blur);
criterion_main!(benches);