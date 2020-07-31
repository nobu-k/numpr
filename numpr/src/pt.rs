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

enum ScanMode {
    Col,
    Row,
    Block,
    All,
}

pub struct PtIter {
    x: usize,
    y: usize,
    i: usize,
    mode: ScanMode,
}

impl PtIter {
    pub fn col(pt: Pt) -> Self {
        Self {
            x: 0,
            y: pt.y,
            i: 0,
            mode: ScanMode::Col,
        }
    }

    pub fn row(pt: Pt) -> Self {
        Self {
            x: pt.x,
            y: 0,
            i: 0,
            mode: ScanMode::Row,
        }
    }

    pub fn block(pt: Pt) -> Self {
        Self {
            x: pt.x / BLOCK_WIDTH * BLOCK_WIDTH,
            y: pt.y / BLOCK_HEIGHT * BLOCK_HEIGHT,
            i: 0,
            mode: ScanMode::Block,
        }
    }

    pub fn all() -> Self {
        Self {
            x: 0,
            y: 0,
            i: 0,
            mode: ScanMode::All,
        }
    }

    pub fn all_after(pt: Pt) -> Self {
        Self {
            x: 0,
            y: 0,
            i: pt.y * WIDTH + pt.x + 1,
            mode: ScanMode::All,
        }
    }
}

impl Iterator for PtIter {
    type Item = Pt;

    fn next(&mut self) -> Option<Self::Item> {
        use ScanMode::*;
        let (x, y) = match self.mode {
            Col if self.i < WIDTH => (self.i, self.y),
            Row if self.i < HEIGHT => (self.x, self.i),
            Block if self.i < 9 => (self.x + self.i % 3, self.y + self.i / 3),
            All if self.i < SIZE => (self.i % WIDTH, self.i / WIDTH),
            _ => return None,
        };
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
