use criterion::{black_box, criterion_group, criterion_main, Criterion};
use numpr::*;

fn place() -> Board {
    // TODO: replace this with the fastest implementation
    placers::PermutationPlacer::new()
        .place(solvers::NaiveSolver::new())
        .unwrap()
}

fn solve(b: Board) {
    // TODO: replace this with the fastest implementation
    solvers::NaiveSolver::new()
        .solve(&b, false)
        .unwrap()
        .validate()
        .unwrap();
}

pub fn generator(c: &mut Criterion) {
    c.bench_function("AllNaive", |b| {
        b.iter(|| {
            let b = place();
            let g = generators::NaiveGenerator::new(black_box(81))
                .generate(
                    &b,
                    || solvers::NaiveSolver::new(),
                    || uniques::NaiveUniquenessChecker::new(),
                )
                .unwrap();
            solve(g);
        });
    });

    c.bench_function("HeuristicSolver-And-Naive", |b| {
        b.iter(|| {
            let b = place();
            let g = generators::NaiveGenerator::new(black_box(81))
                .generate(
                    &b,
                    || solvers::HeuristicSolver::new(),
                    || uniques::NaiveUniquenessChecker::new(),
                )
                .unwrap();
            solve(g);
        });
    });
}

criterion_group!(benches, generator);
criterion_main!(benches);
