use crate::board::Board;
use crate::error::NumprResult;
use crate::solver::Solver;

pub trait UniquenessChecker {
    fn check<S>(self, board: &Board, factory: impl Fn() -> S) -> NumprResult<()>
    where
        S: Solver;
}
