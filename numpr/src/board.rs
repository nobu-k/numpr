use crate::consts::*;
use crate::pt::{Pt, PtIter};

#[derive(Copy, Clone)]
pub struct Board {
    numbers: [u8; SIZE], // TODO: use 4 bits per square
}

impl Board {
    pub fn new(n: &[u8]) -> Result<Board, String> {
        if n.len() != SIZE {
            return Err(format!(
                "slice must have length of {}: len = {}",
                SIZE,
                n.len()
            ));
        }

        let mut b = [0; SIZE];
        b.copy_from_slice(n);
        if let Some((p, v)) = b.iter().enumerate().find(|(_, &v)| v > 9) {
            return Err(format!(
                "invalid value at ({}, {}): {}",
                p % WIDTH,
                p / HEIGHT,
                v
            ));
        }
        return Ok(Board { numbers: b });
    }

    pub fn default() -> Self {
        Self { numbers: [0; SIZE] }
    }

    pub fn get(&self, pt: Pt) -> Option<u8> {
        let v = self.numbers[pt.index()];
        if v == 0 {
            None
        } else {
            Some(v)
        }
    }

    pub fn set(&mut self, pt: Pt, n: u8) -> Result<(), String> {
        if n > 9 {
            return Err(format!("invalid value: {}", n));
        }
        self.numbers[pt.index()] = n;
        Ok(())
    }

    pub fn placeable(&self, pt: Pt, n: u8) -> bool {
        // TODO: see if there's any performance difference between this and
        // col.all(&pred) && row.all && block.all
        self.get(pt).is_none()
            && PtIter::col(pt)
                .chain(PtIter::row(pt))
                .chain(PtIter::block(pt))
                .all(|p| self.get(p) != Some(n)) // TODO: change this to raw get
    }

    pub fn candidates(&self, pt: Pt) -> Candidates {
        Candidates { b: self, n: 1, pt }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            pt: PtIter::all(),
            b: self,
        }
    }

    pub fn iter_after(&self, pt: Pt) -> Iter {
        Iter {
            pt: PtIter::all_after(pt),
            b: self,
        }
    }
}

pub struct Iter<'a> {
    pt: PtIter,
    b: &'a Board,
}

impl<'a> Iterator for Iter<'a> {
    type Item = (Pt, Option<u8>);
    fn next(&mut self) -> Option<Self::Item> {
        let pt = self.pt.next()?;
        Some((pt, self.b.get(pt)))
    }
}

pub struct Candidates<'a> {
    pt: Pt,
    n: u8,
    b: &'a Board,
}

impl<'a> Iterator for Candidates<'a> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        // TODO: replace this implementation by precomputing possible numbers
        // by bitwise or after setting up benchmarks.
        while self.n <= 9 {
            let n = self.n;
            self.n += 1;
            if self.b.placeable(self.pt, n) {
                return Some(n);
            }
        }
        None
    }
}

// TODO: implement a candidate iterator with [u8; 9] and its slice &[u8] to
// avoid allocation (after setting up benchmark).

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "slice must have length of 81")]
    fn empty() {
        Board::new(&[]).unwrap();
    }

    #[test]
    #[should_panic(expected = "slice must have length of 81")]
    fn too_short() {
        Board::new(&[0; SIZE - 1]).unwrap();
    }

    #[test]
    #[should_panic(expected = "slice must have length of 81")]
    fn too_long() {
        Board::new(&[0; SIZE + 1]).unwrap();
    }

    #[test]
    #[should_panic(expected = "invalid value at (2, 1)")]
    fn invalid_value() {
        let mut n = [0; SIZE];
        n[11] = 10;
        Board::new(&n).unwrap();
    }

    #[test]
    fn test_new() {
        let n: Vec<u8> = (1..=SIZE)
            .map(|_| (rand::random::<f64>() * 10.) as u8)
            .collect();

        let b = Board::new(&n).unwrap();
        assert_eq!(b.numbers.len(), SIZE);
        assert!(b.numbers.iter().eq(n.iter()));

        for (p, v) in b.iter() {
            assert_eq!(n[p.index()], v.unwrap_or(0));
            assert_eq!(v, b.get(p))
        }
    }

    #[test]
    fn pt_get() {
        let x = (rand::random::<f64>() * 9.) as usize;
        let y = (rand::random::<f64>() * 9.) as usize;
        let pt = Pt::new(x, y).unwrap();
        assert_eq!(x, pt.x());
        assert_eq!(y, pt.y());
    }

    #[test]
    fn pt_out_of_bounds() {
        assert!(Pt::new(0, 0).is_ok());
        assert!(Pt::new(WIDTH, 0).is_err());
        assert!(Pt::new(1000, 0).is_err());
        assert!(Pt::new(0, HEIGHT).is_err());
        assert!(Pt::new(0, 1000).is_err());
    }

    #[test]
    fn set() {
        let mut b = Board::new(&[1; SIZE]).unwrap();
        let pt = Pt::new(1, 2).unwrap();
        b.set(pt, 3).unwrap();
        assert_eq!(Some(3), b.get(pt));
        b.set(pt, 9).unwrap();
        assert_eq!(Some(9), b.get(pt));
        b.set(pt, 0).unwrap();
        assert_eq!(None, b.get(pt));

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let n = (rand::random::<f64>() * 10.) as u8;
                let pt = Pt::new(x, y).unwrap();
                b.set(pt, n).unwrap();
                if n == 0 {
                    assert_eq!(None, b.get(pt));
                } else {
                    assert_eq!(Some(n), b.get(pt));
                }
            }
        }
    }

    #[test]
    #[should_panic(expected = "invalid value: 10")]
    fn set_invalid_value() {
        let mut b = Board::new(&[1; SIZE]).unwrap();
        b.set(Pt::new(2, 3).unwrap(), 10).unwrap();
    }
}
