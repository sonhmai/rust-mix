use criterion::{black_box, criterion_group, criterion_main, Criterion};
use performance::read_buffered_allocate_string_every_time;

fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("simple", |b| b.iter(|| read_buffered_allocate_string_every_time()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
