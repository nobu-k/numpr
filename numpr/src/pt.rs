use crate::consts::*;
use crate::error::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pt {
    x: usize,
    y: usize,
}

impl Pt {
    pub fn new(x: usize, y: usize) -> NumprResult<Self> {
        if x >= WIDTH || y >= HEIGHT {
            return NumprError::index_out_of_bounds(x, y);
        }
        Ok(Pt { x, y })
    }

    pub fn x(&self) -> usize {
        self.x
    }

    pub fn y(&self) -> usize {
        self.y
    }

    pub fn index(&self) -> usize {
        self.y * HEIGHT + self.x
    }
}

pub struct PtIter {}

impl PtIter {
    pub fn col(pt: Pt) -> ColIter {
        ColIter { x: pt.x, y: 0 }
    }

    pub fn row(pt: Pt) -> RowIter {
        RowIter { x: 0, y: pt.y }
    }

    pub fn block(pt: Pt) -> BlockIter {
        BlockIter {
            x: pt.x / BLOCK_WIDTH * BLOCK_WIDTH,
            y: pt.y / BLOCK_HEIGHT * BLOCK_HEIGHT,
            i: 0,
        }
    }

    pub fn all() -> AllIter {
        AllIter { i: 0 }
    }

    pub fn all_after(pt: Pt) -> AllIter {
        AllIter {
            i: pt.y * WIDTH + pt.x + 1,
        }
    }
}

pub struct ColIter {
    x: usize,
    y: usize,
}

impl Iterator for ColIter {
    type Item = Pt;
    fn next(&mut self) -> Option<Self::Item> {
        if self.y == HEIGHT {
            return None;
        }

        self.y += 1;
        Some(Pt {
            x: self.x,
            y: self.y - 1,
        })
    }
}

pub struct RowIter {
    x: usize,
    y: usize,
}

impl Iterator for RowIter {
    type Item = Pt;
    fn next(&mut self) -> Option<Self::Item> {
        if self.x == WIDTH {
            return None;
        }

        self.x += 1;
        Some(Pt {
            x: self.x - 1,
            y: self.y,
        })
    }
}

pub struct BlockIter {
    x: usize,
    y: usize,
    i: usize,
}

impl Iterator for BlockIter {
    type Item = Pt;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == BLOCK_SIZE {
            return None;
        }

        let x = self.x + self.i % 3;
        let y = self.y + self.i / 3;
        self.i += 1;
        Some(Pt { x, y })
    }
}

pub struct AllIter {
    i: usize,
}

impl Iterator for AllIter {
    type Item = Pt;
    fn next(&mut self) -> Option<Self::Item> {
        if self.i == SIZE {
            return None;
        }

        let x = self.i % WIDTH;
        let y = self.i / WIDTH;
        self.i += 1;
        Some(Pt { x, y })
    }
}

#[cfg(test)]
mod testsss {
    use super::*;

    // Pt is mostly tested by other components such as Board and Solvers, so it
    // doesn't need many tests.
    #[test]
    fn block() {
        let mut expected = vec![vec![vec![]; 3]; 3];
        for pt in PtIter::all() {
            expected[pt.y / BLOCK_HEIGHT][pt.x / BLOCK_WIDTH].push(pt);
        }

        for pt in PtIter::all() {
            assert!(
                PtIter::block(pt).eq(expected[pt.y / BLOCK_HEIGHT][pt.x / BLOCK_WIDTH]
                    .iter()
                    .cloned())
            );
        }
    }
}
