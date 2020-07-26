use crate::board::Board;
use crate::solver::Solver;

pub trait Placer {
    fn place(self, solver: impl Solver) -> Result<Board, String>;
}
