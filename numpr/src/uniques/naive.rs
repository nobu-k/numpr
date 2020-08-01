use crate::board::Board;
use crate::error::{NumprError, NumprResult};
use crate::solver::Solver;
use crate::unique::UniquenessChecker;

pub struct NaiveUniquenessChecker {}

impl NaiveUniquenessChecker {
    pub fn new() -> Self {
        Self {}
    }
}

impl UniquenessChecker for NaiveUniquenessChecker {
    fn check<S>(self, board: &Board, factory: impl Fn() -> S) -> NumprResult<()>
    where
        S: Solver,
    {
        let mut b = *board;
        for (pt, _) in board.iter().filter(|(_, n)| n.is_none()) {
            let mut next = 0;
            for c in board.candidates(pt, false) {
                b.set(pt, c)?;
                let s = factory();
                if s.solve(&b, false).is_ok() {
                    if next != 0 {
                        return NumprError::multiple_solutions();
                    }
                    next = c;
                }
            }
            if next == 0 {
                return NumprError::unsolvable();
            }
            b.set(pt, next)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::NaiveSolver;

    #[test]
    fn default() {
        let b = Board::default();
        let u = NaiveUniquenessChecker::new();
        let err = u.check(&b, || NaiveSolver::new());
        assert!(err.is_err());
    }

    #[test]
    fn solved() {
        let b = Board::default();
        let b = NaiveSolver::new().solve(&b, true).unwrap();
        let u = NaiveUniquenessChecker::new();
        u.check(&b, || NaiveSolver::new()).unwrap();
    }

    #[test]
    fn one_empty_grid() {
        let b = Board::default();
        let mut b = NaiveSolver::new().solve(&b, true).unwrap();

        let p = || (rand::random::<f64>() * 9.) as usize;
        b.set(crate::pt::Pt::new(p(), p()).unwrap(), 0).unwrap();

        let u = NaiveUniquenessChecker::new();
        u.check(&b, || NaiveSolver::new()).unwrap();
    }
}
