use crate::pt::Pt;
use crate::Board;
use crate::Solver;

pub struct NaiveSolver {}

fn empty_grid((_, n): &(Pt, Option<u8>)) -> bool {
    n.is_none()
}

impl NaiveSolver {
    pub fn new() -> Self {
        Self {}
    }

    fn recurse(&self, b: &mut Board, pt: Pt, random: bool) -> Result<Board, String> {
        // Note: precomputing pt and passing it as &[Pt] and [1..] was slower than the current code.
        let next = b.iter_after(pt).find(empty_grid);

        for n in b.candidates(pt, random) {
            b.set(pt, n).unwrap();
            if next.is_none() {
                return Ok(*b);
            }

            let res = self.recurse(b, next.unwrap().0, random);
            if res.is_ok() {
                return res;
            }
        }
        b.set(pt, 0)?;
        return Err("no answer found".to_string());
    }
}

impl Solver for NaiveSolver {
    fn solve(self, board: &Board, random: bool) -> Result<Board, String> {
        let mut b = *board;
        match &mut board.iter().find(empty_grid) {
            Some((pt, _)) => self.recurse(&mut b, *pt, random),
            None => Ok(*board),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solve() {
        let b = Board::default();
        let s = NaiveSolver::new();

        let b = s.solve(&b, false).unwrap();
        b.validate().unwrap();

        let s = NaiveSolver::new();
        let b2 = s.solve(&b, false).unwrap(); // To cover None case in solve
        assert!(b.iter().eq(b2.iter()));
    }

    #[test]
    fn solve_random() {
        let b = Board::default();
        let s = NaiveSolver::new();

        let b = s.solve(&b, true).unwrap();
        b.validate().unwrap();
    }
}
