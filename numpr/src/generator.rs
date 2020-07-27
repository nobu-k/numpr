use crate::board::Board;
use crate::solver::Solver;
use crate::unique::UniquenessChecker;

pub trait Generator {
    fn generate<S, U>(
        self,
        board: &Board,
        solver_factory: impl Fn() -> S,
        unique_factory: impl Fn() -> U,
    ) -> Result<Board, String>
    where
        S: Solver,
        U: UniquenessChecker;
}
