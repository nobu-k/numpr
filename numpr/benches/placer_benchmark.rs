use criterion::{black_box, criterion_group, criterion_main, Criterion};
use numpr::{NaivePlacer, NaiveSolver, Placer};

pub fn placer(c: &mut Criterion) {
    c.bench_function("NaivePlacer-NaiveSolver", |b| {
        b.iter(|| {
            let p = NaivePlacer {};
            let s = NaiveSolver::new();
            let b = p.place(s).unwrap();
            b.validate().unwrap();
        });
    });
}

criterion_group!(benches, placer);
criterion_main!(benches);
