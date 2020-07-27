use crate::board::Board;
use crate::solver::Solver;

pub trait UniquenessChecker {
    fn check<S>(self, board: &Board, factory: impl Fn() -> S) -> Result<(), String>
    where
        S: Solver;
}
