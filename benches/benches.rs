use ::spellit::phonetic_alphabets_of;
use criterion::{black_box, criterion_group, criterion_main, Criterion};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("phonetic alphabets of", |b| {
        b.iter(|| phonetic_alphabets_of(black_box("Lorem ipsum dolor sit amet, consectetur adipiscing elit.\nPellentesque pretium tincidunt lectus nisi.")))
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
