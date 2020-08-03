use crate::consts::*;
use crate::error::*;

// A struct to hold a coordinate on a `[Board](struct.Board.html)`.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Pt {
    x: usize,
    y: usize,
}

impl Pt {
    /// Returns a new point create from `x` and `y`. Coordinates start at `0`.
    ///
    /// It fails and returns `NumprError::IndexOutOfBounds` when `x` or `y` is
    /// not within `[0, 8]`.
    pub fn new(x: usize, y: usize) -> NumprResult<Self> {
        if x >= WIDTH || y >= HEIGHT {
            return NumprError::index_out_of_bounds(x, y);
        }
        Ok(Pt { x, y })
    }

    /// Returns `x` coordinate of the point.
    pub fn x(&self) -> usize {
        self.x
    }

    /// Returns `y` coordinate of the point.
    pub fn y(&self) -> usize {
        self.y
    }

    /// Returns an index corresponding to the point in an one dimensional array.
    ///
    /// # Example
    ///
    /// ```
    /// use numpr::{Pt, WIDTH, HEIGHT};
    /// let (x, y) = (3, 5);
    /// let i = Pt::new(x, y).unwrap().index();
    /// assert_eq!(i, 48);
    /// assert_eq!(i % WIDTH, x);
    /// assert_eq!(i / WIDTH, y);
    /// ```
    pub fn index(&self) -> usize {
        self.y * HEIGHT + self.x
    }
}

/// An iterator to traverse on a `[Board](struct.Board.html)`.
///
/// PtIter itself doesn't have any capability to iterator over boards. Create
/// related iterators using its static methods.
pub struct PtIter {
    _dummy: (),
}

impl PtIter {
    /// Returns an iterator to iterates over the column containing the `pt`.
    ///
    /// # Example
    ///
    /// ```
    /// use numpr::{Pt, PtIter};
    /// let mut it = PtIter::col(Pt::new(3, 5).unwrap());
    /// assert_eq!(it.next(), Some(Pt::new(3, 0).unwrap()));
    /// assert_eq!(it.next(), Some(Pt::new(3, 1).unwrap()));
    /// assert_eq!(it.last(), Some(Pt::new(3, 8).unwrap()));
    /// ```
    pub fn col(pt: Pt) -> ColIter {
        ColIter { x: pt.x, y: 0 }
    }

    /// Returns an iterator to iterates over the row containing the `pt`.
    ///
    /// # Example
    ///
    /// ```
    /// use numpr::{Pt, PtIter};
    /// let mut it = PtIter::row(Pt::new(3, 5).unwrap());
    /// assert_eq!(it.next(), Some(Pt::new(0, 5).unwrap()));
    /// assert_eq!(it.next(), Some(Pt::new(1, 5).unwrap()));
    /// assert_eq!(it.last(), Some(Pt::new(8, 5).unwrap()));
    /// ```
    pub fn row(pt: Pt) -> RowIter {
        RowIter { x: 0, y: pt.y }
    }

    /// Returns an iterator to iterates over the block containing the `pt`.
    ///
    /// # Example
    ///
    /// ```
    /// use numpr::{Pt, PtIter};
    /// let mut it = PtIter::block(Pt::new(3, 5).unwrap());
    /// assert_eq!(it.next(), Some(Pt::new(3, 3).unwrap()));
    /// assert_eq!(it.next(), Some(Pt::new(4, 3).unwrap()));
    /// assert_eq!(it.last(), Some(Pt::new(5, 5).unwrap()));
    /// ```
    pub fn block(pt: Pt) -> BlockIter {
        BlockIter {
            x: pt.x / BLOCK_WIDTH * BLOCK_WIDTH,
            y: pt.y / BLOCK_HEIGHT * BLOCK_HEIGHT,
            i: 0,
        }
    }

    /// Returns an iterator to iterates over an entire board. The iteration
    /// starts at `(0, 0)`.
    ///
    /// # Example
    ///
    /// ```
    /// use numpr::{Pt, PtIter};
    /// let mut it = PtIter::all();
    /// assert_eq!(it.next(), Some(Pt::new(0, 0).unwrap()));
    /// assert_eq!(it.next(), Some(Pt::new(1, 0).unwrap()));
    /// assert_eq!(it.last(), Some(Pt::new(8, 8).unwrap()));
    /// ```
    pub fn all() -> AllIter {
        AllIter { i: 0 }
    }

    /// Returns an iterator to iterates over an entire board starting at the
    /// next grid of `pt`.
    ///
    /// # Example
    ///
    /// ```
    /// use numpr::{Pt, PtIter};
    /// let mut it = PtIter::all_after(Pt::new(3, 5).unwrap());
    /// assert_eq!(it.next(), Some(Pt::new(4, 5).unwrap()));
    /// assert_eq!(it.next(), Some(Pt::new(5, 5).unwrap()));
    /// assert_eq!(it.last(), Some(Pt::new(8, 8).unwrap()));
    /// ```
    pub fn all_after(pt: Pt) -> AllIter {
        AllIter {
            i: pt.y * WIDTH + pt.x + 1,
        }
    }
}

/// An iterator to iterates over a column.
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

/// An iterator to iterates over a row.
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

/// An iterator to iterates over a 3x3 block.
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

/// An iterator to iterates over an entire board.
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
mod tests {
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
