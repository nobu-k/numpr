use crate::consts::*;
use crate::error::{NumprError, NumprResult};
use crate::pt::{Pt, PtIter};
use crate::Board;
use crate::Solver;

pub struct HeuristicSolver {
    masks: [u16; SIZE],
    popcnts: [u8; SIZE],
}

impl HeuristicSolver {
    pub fn new() -> Self {
        Self {
            masks: [0b11_1111_1110; SIZE],
            popcnts: [9; SIZE],
        }
    }

    fn init(&mut self, b: &Board) -> NumprResult<usize> {
        let mut cnt = 0;
        for (pt, n) in b.iter().filter(|(_, n)| n.is_some()) {
            cnt += 1;
            let n = n.unwrap();
            self.masks[pt.index()] = 0;
            self.popcnts[pt.index()] = 10;
            self.dec(pt, n)?;
        }
        Ok(SIZE - cnt)
    }

    fn set(&mut self, b: &mut Board, pt: Pt, n: u8) -> NumprResult<()> {
        b.set(pt, n)?;
        self.masks[pt.index()] = 0;
        self.popcnts[pt.index()] = 10;
        self.dec(pt, n)?;
        Ok(())
    }

    fn dec(&mut self, pt: Pt, n: u8) -> NumprResult<()> {
        let it = PtIter::col(pt)
            .chain(PtIter::row(pt))
            .chain(PtIter::block(pt))
            .filter(|p| *p != pt);
        for p in it {
            // TODO: check if this skip is really fast
            if self.popcnts[p.index()] == 10 {
                continue;
            }

            let b = 1 << n;
            let m = (self.masks[p.index()] & b) != 0;
            self.masks[p.index()] &= !b;
            self.popcnts[p.index()] -= m as u8;
            if self.popcnts[p.index()] == 0 {
                return NumprError::unsolvable();
            }
        }
        Ok(())
    }

    fn recurse(&mut self, b: &mut Board, mut idx: &mut [u8], random: bool) -> NumprResult<Board> {
        let mut prev_cnt = 0;
        loop {
            idx[..].sort_by(|l, r| self.popcnts[*l as usize].cmp(&self.popcnts[*r as usize]));
            let len = idx.len();
            idx = &mut idx[..(len - prev_cnt)]; // TODO: fix this to work
            if idx.is_empty() || self.popcnts[idx[0] as usize] != 1 {
                break;
            }

            prev_cnt = 0;
            for i in &idx[..] {
                let i = *i as usize;
                if self.popcnts[i] != 1 {
                    break;
                }
                prev_cnt += 1;
                // TODO: implement fast log2
                for bit in 1..=9 {
                    if (self.masks[i] >> bit) == 1 {
                        self.set(b, Pt::new(i % WIDTH, i / WIDTH)?, bit)?;
                        break;
                    }
                }
            }
        }
        if idx.is_empty() {
            return Ok(*b);
        }

        let next = idx[0] as usize;
        if self.popcnts[next] == 10 {
            return Ok(*b);
        }

        let pt = Pt::new(next % WIDTH, next / WIDTH)?;
        for c in b.candidates(pt, random) {
            let mut b = *b;
            let masks = self.masks;
            let popcnts = self.popcnts;
            if self.set(&mut b, pt, c).is_ok() {
                let res = self.recurse(&mut b, idx, random);
                if res.is_ok() {
                    return res;
                }
            }
            self.masks = masks;
            self.popcnts = popcnts;
        }
        NumprError::unsolvable()
    }
}

impl Solver for HeuristicSolver {
    fn solve(mut self, board: &Board, random: bool) -> NumprResult<Board> {
        let mut idx = [0u8; SIZE];
        for i in 0..SIZE {
            idx[i] = i as u8;
        }

        let cnt = self.init(&board)?;
        if cnt == 0 {
            return Ok(*board);
        }
        idx[..].sort_by(|l, r| self.popcnts[*l as usize].cmp(&self.popcnts[*r as usize]));
        let mut board = *board;
        self.recurse(&mut board, &mut idx[..cnt], random)
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
        b.validate().unwrap();
    }

    #[test]
    fn partially_solve() {
        let b = NaivePlacer::new().place(NaiveSolver::new()).unwrap();
        let b = NaiveGenerator::new(crate::consts::SIZE as u32)
            .generate(&b, || NaiveSolver::new(), || NaiveUniquenessChecker::new())
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
