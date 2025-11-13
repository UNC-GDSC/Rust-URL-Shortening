use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rust_url_shortener::utils::generate_short_code;

fn benchmark_short_code_generation(c: &mut Criterion) {
    c.bench_function("generate_short_code_6", |b| {
        b.iter(|| generate_short_code(black_box(6)))
    });

    c.bench_function("generate_short_code_8", |b| {
        b.iter(|| generate_short_code(black_box(8)))
    });

    c.bench_function("generate_short_code_12", |b| {
        b.iter(|| generate_short_code(black_box(12)))
    });
}

criterion_group!(benches, benchmark_short_code_generation);
criterion_main!(benches);
