use crate::board::Board;
use crate::placer::Placer;
use crate::solver::Solver;

pub struct NaivePlacer {}

impl Placer for NaivePlacer {
    fn place(self, solver: impl Solver) -> Result<Board, String> {
        solver.solve(&Board::default(), true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place() {
        let p = NaivePlacer {};
        let b = p.place(crate::NaiveSolver::new()).unwrap();
        b.validate().unwrap();
    }
}
