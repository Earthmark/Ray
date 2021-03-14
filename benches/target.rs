use criterion::{black_box, criterion_group, criterion_main, Criterion};
use glam::{UVec2, Vec3};
use ray::range::range;
use ray::target::ImageTarget;

fn iterator_access(c: &mut Criterion) {
    let dims = UVec2::new(1024, 1024);
    let mut target = ImageTarget::new(dims, Vec3::new(0.0, 0.0, 0.0));

    c.bench_function("write through direct index access", |b| {
        b.iter(|| {
            for x in 0..1024 {
                for y in 0..1024 {
                    *target.get_mut(UVec2::new(x, y)).unwrap() =
                        black_box(Vec3::new(x as f32, y as f32, 0.0));
                }
            }
        })
    });
    c.bench_function("write through range iterator", |b| {
        b.iter(|| {
            for index in range(UVec2::new(1024, 1024)) {
                if let Some(pixel) = target.get_mut(index) {
                    *pixel = black_box(Vec3::new(index.x as f32, index.y as f32, 0.0));
                }
            }
        })
    });
}

fn raw_array_iterator_access(c: &mut Criterion) {
    let mut data = vec![Vec3::ZERO; 1024 * 1024];

    c.bench_function("raw array write through direct array indexed access", |b| {
        b.iter(|| {
            for i in 0..(1024 * 1024) {
                *data.get_mut(black_box(i)).unwrap() =
                    black_box(Vec3::new(i as f32, i as f32, 0.0));
            }
        })
    });
    c.bench_function("raw array write through iterator range access", |b| {
        b.iter(|| {
            for pixel in data.iter_mut() {
                *pixel = black_box(Vec3::new(1.0, 1.0, 0.0));
            }
        })
    });
}

criterion_group!(props_benches, iterator_access, raw_array_iterator_access);
criterion_main!(props_benches);
