use crate::board::Board;
use crate::consts::*;
use crate::placer::Placer;
use crate::pt::Pt;
use crate::solver::Solver;
use rand::prelude::*;

pub struct PermutationPlacer {}

impl PermutationPlacer {
    pub fn new() -> Self {
        Self {}
    }

    fn fill_upper_blocks(&self, b: &mut Board) -> Result<(), String> {
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

    // TODO: this make the performance 2000% worse somehow!!
    // check why it happens.
    fn fill_left_blocks(&self, b: &mut Board) -> Result<(), String> {
        let rng = &mut rand::thread_rng();

        // Fill the 1st col.
        let mut mask = 0;
        for y in 0..3 {
            mask |= 1 << b.raw_get(Pt::new(0, y)?);
        }
        let mut init = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];

        // Remove numbers already used in the top block.
        let mut k = 0;
        for i in 0..9 {
            init[k] = init[i];
            if (mask & (1 << init[i])) == 0 {
                k += 1;
            }
        }
        init[..6].shuffle(rng);

        // TODO: 0..1 ~ 0..2 are fine, but 0..3 or more makes it super slow
        // Because of some operations in Board?
        for y in 0..6 {
            b.set(Pt::new(0, y + 3)?, init[y])?;
        }

        // TODO: fill remaining columns.
        Ok(())
    }
}

impl Placer for PermutationPlacer {
    fn place(self, solver: impl Solver) -> Result<Board, String> {
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
        let b = p.place(crate::NaiveSolver::new()).unwrap();
        b.validate().unwrap();
    }
}
