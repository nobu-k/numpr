use crate::board::Board;
use crate::error::NumprResult;
use crate::placer::Placer;
use crate::solver::Solver;

pub struct NaivePlacer {}

impl NaivePlacer {
    pub fn new() -> Self {
        Self {}
    }
}

impl Placer for NaivePlacer {
    fn place(self, solver: impl Solver) -> NumprResult<Board> {
        solver.solve(&Board::default(), true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place() {
        let p = NaivePlacer::new();
        let b = p.place(crate::NaiveSolver::new()).unwrap();
        b.validate().unwrap();
    }
}
