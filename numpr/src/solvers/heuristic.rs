use crate::consts::*;
use crate::error::{NumprError, NumprResult};
use crate::pt::{Pt, PtIter};
use crate::Board;
use crate::Solver;

pub struct HeuristicSolver {
    masks: [u16; SIZE],
}

const LOG_TABLE_LOW: [u8; 9] = [0, 1, 2, 0, 3, 0, 0, 0, 4];
const LOG_TABLE_HIGH: [u8; 9] = [0, 5, 6, 0, 7, 0, 0, 0, 8];

impl HeuristicSolver {
    pub fn new() -> Self {
        Self {
            masks: [0b11_1111_1110; SIZE],
        }
    }

    fn init(&mut self, b: &Board) -> NumprResult<usize> {
        let mut cnt = 0;
        for (pt, n) in b.iter().filter(|(_, n)| n.is_some()) {
            cnt += 1;
            let n = n.unwrap();
            self.masks[pt.index()] = 1;
            self.dec(pt, n)?;
        }
        Ok(SIZE - cnt)
    }

    fn set(&mut self, b: &mut Board, pt: Pt, n: u8) -> NumprResult<()> {
        b.set(pt, n)?;
        self.masks[pt.index()] = 1;
        self.dec(pt, n)?;
        Ok(())
    }

    fn dec(&mut self, pt: Pt, n: u8) -> NumprResult<()> {
        let b = !(1 << n);
        for p in PtIter::row(pt) {
            self.masks[p.index()] &= b;
            if self.masks[p.index()] == 0 {
                return NumprError::unsolvable();
            }
        }

        for p in PtIter::block(pt) {
            self.masks[p.index()] &= b;
            if self.masks[p.index()] == 0 {
                return NumprError::unsolvable();
            }
        }

        for p in PtIter::col(pt) {
            self.masks[p.index()] &= b;
            if self.masks[p.index()] == 0 {
                return NumprError::unsolvable();
            }
        }
        Ok(())
    }

    fn recurse(&mut self, b: &mut Board, mut idx: &mut [u8], random: bool) -> NumprResult<Board> {
        loop {
            if idx.is_empty() {
                break;
            }

            let mut k = 0;
            let mut end = idx.len();
            for _ in 0..idx.len() {
                let i = idx[k] as usize;

                // Is only one bit (i.e. one number to put) left?
                let m = self.masks[i] >> 1;
                if m & (m - 1) != 0 {
                    k += 1;
                    continue;
                }

                end -= 1;
                idx.swap(k, end);

                let bit = LOG_TABLE_LOW[(m & 0xf) as usize]
                    + LOG_TABLE_HIGH[((m >> 4) & 0xf) as usize]
                    + ((m >> 8) * 9) as u8;
                self.set(b, Pt::new(i % WIDTH, i / WIDTH)?, bit)?;
            }
            if end == idx.len() {
                break;
            }
            idx = &mut idx[..end];
        }
        if idx.is_empty() {
            return Ok(*b);
        }

        let back = idx.len() - 1;
        let next = idx[back] as usize;
        idx = &mut idx[..back];

        let pt = Pt::new(next % WIDTH, next / WIDTH)?;
        for c in b.candidates(pt, random) {
            let mut b = *b;
            let masks = self.masks;
            if self.set(&mut b, pt, c).is_ok() {
                let res = self.recurse(&mut b, idx, random);
                if res.is_ok() {
                    return res;
                }
            }
            self.masks = masks;
        }
        NumprError::unsolvable()
    }
}

impl Solver for HeuristicSolver {
    fn solve(mut self, board: &Board, random: bool) -> NumprResult<Board> {
        let cnt = self.init(&board)?;
        if cnt == 0 {
            return Ok(*board);
        }

        let mut idx = [0u8; SIZE];
        let idx = &mut idx[..cnt];

        {
            let mut k = 0;
            for i in 0..SIZE {
                if self.masks[i] == 1 {
                    continue;
                }
                idx[k] = i as u8;
                k += 1;
            }
        }

        let mut board = *board;
        self.recurse(&mut board, idx, random)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn solve() {
        let b = Board::default();
        let s = HeuristicSolver::new();

        let b = s.solve(&b, false).unwrap();
        for y in 0..9 {
            for x in 0..9 {
                print!("{} ", b.get(Pt::new(x, y).unwrap()).unwrap_or(0));
            }
            println!("");
        }
        b.validate().unwrap();
    }

    #[test]
    fn partially_solve() {
        let b = placers::NaivePlacer::new()
            .place(solvers::NaiveSolver::new())
            .unwrap();
        let b = generators::NaiveGenerator::new(crate::consts::SIZE as u32)
            .generate(
                &b,
                || solvers::NaiveSolver::new(),
                || uniques::NaiveUniquenessChecker::new(),
            )
            .unwrap();

        let s = HeuristicSolver::new();

        let b = s.solve(&b, false).unwrap();
        for y in 0..9 {
            for x in 0..9 {
                print!("{} ", b.get(Pt::new(x, y).unwrap()).unwrap_or(0));
            }
            println!("");
        }
        b.validate().unwrap();
    }
}
