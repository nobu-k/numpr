use crate::board::Board;
use crate::error::NumprResult;
use crate::solver::Solver;

pub trait Placer {
    fn place(self, solver: impl Solver) -> NumprResult<Board>;
}
