use criterion::{black_box, criterion_group, criterion_main, Criterion};
use numpr::*;

fn place() -> Board {
    // TODO: replace this with the fastest implementation
    PermutationPlacer::new().place(NaiveSolver::new()).unwrap()
}

fn solve(b: Board) {
    // TODO: replace this with the fastest implementation
    NaiveSolver::new()
        .solve(&b, false)
        .unwrap()
        .validate()
        .unwrap();
}

pub fn generator(c: &mut Criterion) {
    c.bench_function("AllNaive", |b| {
        b.iter(|| {
            let b = place();
            let g = NaiveGenerator::new(black_box(81))
                .generate(&b, || NaiveSolver::new(), || NaiveUniquenessChecker::new())
                .unwrap();
            solve(g);
        });
    });

    c.bench_function("HeuristicSolver-And-Naive", |b| {
        b.iter(|| {
            let b = place();
            let g = NaiveGenerator::new(black_box(81))
                .generate(
                    &b,
                    || HeuristicSolver::new(),
                    || NaiveUniquenessChecker::new(),
                )
                .unwrap();
            solve(g);
        });
    });
}

criterion_group!(benches, generator);
criterion_main!(benches);
