use crate::board::Board;
use crate::generator::Generator;
use crate::pt::{Pt, PtIter};
use crate::solver::Solver;
use crate::unique::UniquenessChecker;
use rand::prelude::*;

pub struct NaiveGenerator {
    max_empty_grid: u32,
}

impl NaiveGenerator {
    pub fn new(max_empty_grid: u32) -> NaiveGenerator {
        NaiveGenerator { max_empty_grid }
    }
}

impl Generator for NaiveGenerator {
    fn generate<S, U>(
        mut self,
        board: &Board,
        solver_factory: impl Fn() -> S,
        unique_factory: impl Fn() -> U,
    ) -> Result<Board, String>
    where
        S: Solver,
        U: UniquenessChecker,
    {
        let mut targets: Vec<Pt> = PtIter::all().collect();
        targets.shuffle(&mut rand::thread_rng());

        let mut b = *board;
        for pt in targets {
            let prev_state = b.get(pt).unwrap();
            b.set(pt, 0)?;

            let u = unique_factory();
            if let Err(_) = u.check(&b, &solver_factory) {
                b.set(pt, prev_state)?;
                continue;
            }

            self.max_empty_grid -= 1;
            if self.max_empty_grid == 0 {
                break;
            }
        }
        Ok(b)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::NaiveSolver;
    use crate::uniques::NaiveUniquenessChecker;

    #[test]
    fn generate() {
        let b = Board::default();
        let b = NaiveSolver::new().solve(&b, true).unwrap();
        let g = NaiveGenerator::new(crate::consts::SIZE as u32)
            .generate(&b, || NaiveSolver::new(), || NaiveUniquenessChecker::new())
            .unwrap();

        let answer = NaiveSolver::new().solve(&g, true).unwrap();
        assert!(b.iter().eq(answer.iter()));
    }
}
