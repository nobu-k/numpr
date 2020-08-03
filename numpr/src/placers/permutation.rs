use crate::board::Board;
use crate::consts::*;
use crate::error::NumprResult;
use crate::placer::Placer;
use crate::pt::Pt;
use crate::solver::Solver;
use rand::prelude::*;

pub struct PermutationPlacer {}

// TODO: refactor methods

impl PermutationPlacer {
    pub fn new() -> Self {
        Self {}
    }

    fn fill_upper_blocks(&self, b: &mut Board) -> NumprResult<()> {
        let rng = &mut rand::thread_rng();

        // Fill the first row
        let mut init = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
        init[..].shuffle(rng);
        for x in 0..WIDTH {
            b.set(Pt::new(x, 0)?, init[x])?;
        }

        // Shuffle within each blocks for further use
        init[0..3].shuffle(rng);
        init[3..6].shuffle(rng);
        init[6..9].shuffle(rng);

        // Fill the 2nd row of the left block. At this point, n [0, 3] numbers
        // are used from the middle block for the left block, and 3 - n are used
        // from the right block.
        let n = (rand::random::<f64>() * 4.) as usize;
        let mut block = [0u8; 3];
        for i in 0..n {
            // Choose n numbers from middle.
            block[i] = init[i + 3];
        }
        for i in n..3 {
            // Choose 3 - n numbers from right. Skip first n numbers here for symmetry.
            block[i] = init[i + 6];
        }
        block[..].shuffle(rng);
        for x in 0..3 {
            b.set(Pt::new(x, 1)?, block[x])?;
        }

        // Fill the 2nd row of the middle block. Because only 3 - n numbers are
        // used from the right bloc for the first block, the middle block needs
        // to consume remaining 3 - (3 - n) = n numbers from the right block as
        // those numbers are already in the right block and cannot be used
        // within the same block. As a consequence, the middle block takes 3 - n
        // numbers from the left block.
        for i in 0..n {
            // The last 3 - n numbers in the right block are already used.
            block[i] = init[i + 6];
        }
        for i in n..3 {
            block[i] = init[i]; // from the left block.
        }
        block[..].shuffle(rng);
        for x in 0..3 {
            b.set(Pt::new(x + 3, 1)?, block[x])?;
        }

        // Take the remaining from the rest of the block for the right block.
        for i in 0..n {
            block[i] = init[i]; // from the left block.
        }
        for i in n..3 {
            block[i] = init[i + 3];
        }
        block[..].shuffle(rng);
        for x in 0..3 {
            b.set(Pt::new(x + 6, 1)?, block[x])?;
        }

        // Fill the 3rd row by taking numbers that weren't chosen in the last run.
        for i in n..3 {
            block[i] = init[i + 3];
        }
        for i in 0..n {
            block[i] = init[i + 6];
        }
        block[..].shuffle(rng);
        for x in 0..3 {
            b.set(Pt::new(x, 2)?, block[x])?;
        }

        for i in n..3 {
            block[i] = init[i + 6];
        }
        for i in 0..n {
            block[i] = init[i]; // from the left block.
        }
        block[..].shuffle(rng);
        for x in 0..3 {
            b.set(Pt::new(x + 3, 2)?, block[x])?;
        }

        for i in n..3 {
            block[i] = init[i]; // from the left block.
        }
        for i in 0..n {
            block[i] = init[i + 3];
        }
        block[..].shuffle(rng);
        for x in 0..3 {
            b.set(Pt::new(x + 6, 2)?, block[x])?;
        }
        Ok(())
    }

    fn fill_left_blocks(&self, b: &mut Board) -> NumprResult<()> {
        let rng = &mut rand::thread_rng();

        // Fill the 1st col.
        let mut mask = 0;
        for y in 0..3 {
            mask |= 1 << b.raw_get(Pt::new(0, y)?);
        }
        let mut init = [0u8; 9];

        // Remove numbers already used in the top block.
        let mut k = 3;
        let mut begin = 0;
        for i in 1u8..=9 {
            if (mask & (1 << i)) == 0 {
                init[k] = i;
                k += 1;
            } else {
                init[begin] = i;
                begin += 1;
            }
        }
        init[3..].shuffle(rng);

        for y in 0..6 {
            b.set(Pt::new(0, y + 3)?, init[y + 3])?;
        }

        // Shuffle within each blocks for further use
        init[0..3].shuffle(rng);
        init[3..6].shuffle(rng);
        init[6..9].shuffle(rng);

        // Check duplicates between the 1st col of the middle block and the 2nd
        // col of the top block. This number works as "n" in fill_upper_blocks.
        // Because 3 - n numbers in the middle block must be placed in the
        // bottom block, the bottom block needs to take n numbers from the top
        // block. As a result, the middle block has to get 3 - n numbers from
        // the top and n numbers from the bottom.
        let mut n = 0;
        let mask = (1 << b.raw_get(Pt::new(1, 0)?))
            | (1 << b.raw_get(Pt::new(1, 1)?))
            | (1 << b.raw_get(Pt::new(1, 2)?));
        for y in 3..6 {
            if mask & (1 << init[y]) != 0 {
                n += 1;
            }
        }
        let n = n;

        // 2nd col of middle block
        let mut block = [0u8; 3];
        let mut k = 0;
        for i in 0..n {
            loop {
                let v = init[k + 6];
                k += 1;
                if mask & (1 << v) == 0 {
                    block[i] = v;
                    break;
                }
            }
        }
        for i in n..3 {
            block[i] = init[i];
        }
        block[..].shuffle(rng);
        for y in 0..3 {
            b.set(Pt::new(1, y + 3)?, block[y])?;
        }

        // 2nd col of third block
        let mut block = [0u8; 3];
        for i in 0..n {
            block[i] = init[i];
        }
        let mut k = 0;
        for i in n..3 {
            loop {
                let v = init[k + 3];
                k += 1;
                if mask & (1 << v) == 0 {
                    block[i] = v;
                    break;
                }
            }
        }
        block[..].shuffle(rng);
        for y in 0..3 {
            b.set(Pt::new(1, y + 6)?, block[y])?;
        }

        // FIXME: Filling the 3rd col somehow slows down NaiveSolver. Also, only
        // filling the 1st col slows it down 2000%. Investigate why.

        /*
        // Fill the 3rd col with Board::candidates.
        for (i, c) in b.candidates(Pt::new(2, 3)?, true).into_iter().enumerate() {
            b.set(Pt::new(2, i + 3)?, c)?;
        }
        for (i, c) in b.candidates(Pt::new(2, 6)?, true).into_iter().enumerate() {
            b.set(Pt::new(2, i + 6)?, c)?;
        }
        */
        Ok(())
    }
}

impl Placer for PermutationPlacer {
    fn place(self, solver: impl Solver) -> NumprResult<Board> {
        let mut b = Board::default();
        self.fill_upper_blocks(&mut b)?;
        self.fill_left_blocks(&mut b)?;
        solver.solve(&b, true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn place() {
        let p = PermutationPlacer::new();
        let b = p.place(crate::solvers::NaiveSolver::new()).unwrap();
        b.validate().unwrap();
    }
}
