use crate::board::Board;
use crate::error::{NumprError, NumprResult};
use crate::solver::Solver;
use crate::unique::UniquenessChecker;

/// Checks if the given puzzle has a unique solution by putting all possible
/// candidates.
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
        // TODO: use hash with something like LRU cache to detect boards that
        // has already been checked. Storing ones having more empty grids should
        // be cached rather than recent usage? Maybe it's worth implementing a
        // new type?
        let mut b = *board;
        for (pt, _) in board.iter().filter(|(_, n)| n.is_none()) {
            let mut found = 0;
            for c in board.candidates(pt, false) {
                b.set(pt, c)?;
                let s = factory();
                if s.solve(&b, false).is_ok() {
                    if found != 0 {
                        return NumprError::multiple_solutions();
                    }
                    found = c;
                }
            }
            if found == 0 {
                return NumprError::unsolvable();
            }

            // Filling the grid with the answer could resolve a possible
            // ambiguity in another grid. So, it has to be reset to vacant.
            b.set(pt, 0)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::solvers::HeuristicSolver;

    #[test]
    fn default() {
        let b = Board::default();
        let u = NaiveUniquenessChecker::new();
        let err = u.check(&b, || HeuristicSolver::new());
        assert!(err.is_err());
    }

    #[test]
    fn solved() {
        let b = Board::default();
        let b = HeuristicSolver::new().solve(&b, true).unwrap();
        let u = NaiveUniquenessChecker::new();
        u.check(&b, || HeuristicSolver::new()).unwrap();
    }

    #[test]
    fn one_empty_grid() {
        let b = Board::default();
        let mut b = HeuristicSolver::new().solve(&b, true).unwrap();

        let p = || (rand::random::<f64>() * 9.) as usize;
        b.set(crate::pt::Pt::new(p(), p()).unwrap(), 0).unwrap();

        let u = NaiveUniquenessChecker::new();
        u.check(&b, || HeuristicSolver::new()).unwrap();
    }
}
