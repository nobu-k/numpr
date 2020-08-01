use crate::board::Board;
use crate::error::NumprResult;
use crate::solver::Solver;
use crate::unique::UniquenessChecker;

pub trait Generator {
    fn generate<S, U>(
        self,
        board: &Board,
        solver_factory: impl Fn() -> S,
        unique_factory: impl Fn() -> U,
    ) -> NumprResult<Board>
    where
        S: Solver,
        U: UniquenessChecker;
}
