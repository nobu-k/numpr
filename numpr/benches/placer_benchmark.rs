use criterion::{criterion_group, criterion_main, Criterion};
use numpr::{
    placers::{NaivePlacer, PermutationPlacer},
    solvers::{HeuristicSolver, NaiveSolver},
    Placer,
};

pub fn placer(c: &mut Criterion) {
    c.bench_function("NaivePlacer-NaiveSolver", |b| {
        b.iter(|| {
            let p = NaivePlacer::new();
            let s = NaiveSolver::new();
            let b = p.place(s).unwrap();
            b.validate().unwrap();
        });
    });
    c.bench_function("PermutationPlacer-NaiveSolver", |b| {
        b.iter(|| {
            let p = PermutationPlacer::new();
            let s = NaiveSolver::new();
            let b = p.place(s).unwrap();
            b.validate().unwrap();
        });
    });

    c.bench_function("NaivePlacer-HeuristicSolver", |b| {
        b.iter(|| {
            let p = NaivePlacer {};
            let s = HeuristicSolver::new();
            let b = p.place(s).unwrap();
            b.validate().unwrap();
        });
    });
    c.bench_function("PermutationPlacer-HeuristicSolver", |b| {
        b.iter(|| {
            let p = PermutationPlacer::new();
            let s = HeuristicSolver::new();
            let b = p.place(s).unwrap();
            b.validate().unwrap();
        });
    });
}

criterion_group!(benches, placer);
criterion_main!(benches);
