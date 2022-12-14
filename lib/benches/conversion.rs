use criterion::{criterion_group, criterion_main, Criterion};
use lib::cube333::coordcube::CoordCube;

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("double convert identity", |b| b.iter(|| {
        let identity = CoordCube.solved();
        identity.to_cubie().to_coord()
    }));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
