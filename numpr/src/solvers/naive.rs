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

    fn recurse(&self, board: &Board, pt: Pt, random: bool) -> Result<Board, String> {
        // TODO: Because candidates is now independent of Board, board.set can
        // directly be called in the for loop below. Because of that, there's
        // no need to make a copy here. Try it after setting up benchmark.
        let mut b = *board;

        // TODO: next can be cached or precomputed.
        // For example, pt can be &[Pt] that contains all empty grids.
        // then recurse with &pt[1..] might be faster than computing this every time.
        let next = board.iter_after(pt).find(empty_grid);

        // TODO: shuffle candidates when random is true.
        for n in board.candidates(pt, random) {
            b.set(pt, n).unwrap();
            if next.is_none() {
                return Ok(b);
            }

            let res = self.recurse(&b, next.unwrap().0, random);
            if res.is_ok() {
                return res;
            }
        }
        return Err("no answer found".to_string());
    }
}

impl Solver for NaiveSolver {
    fn solve(self, board: &Board, random: bool) -> Result<Board, String> {
        match &mut board.iter().find(empty_grid) {
            Some((pt, _)) => self.recurse(board, *pt, random),
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
