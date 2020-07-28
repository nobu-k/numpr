use crate::consts::*;
use crate::pt::{Pt, PtIter};
use rand::prelude::*;

// Note: using 4-bits per grid didn't improve the performance at least for
// NaiveSolver.

#[derive(Copy, Clone)]
pub struct Board {
    numbers: [u8; SIZE],
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

    fn raw_get(&self, pt: Pt) -> u8 {
        self.numbers[pt.index()]
    }

    pub fn set(&mut self, pt: Pt, n: u8) -> Result<(), String> {
        if n > 9 {
            return Err(format!("invalid value: {}", n));
        }
        self.numbers[pt.index()] = n;
        Ok(())
    }

    pub fn placeable(&self, pt: Pt, n: u8) -> bool {
        let f = |p| self.raw_get(p) != n;
        n > 0
            && self.raw_get(pt) == 0
            && PtIter::col(pt).all(f)
            && PtIter::row(pt).all(f)
            && PtIter::block(pt).all(f)
    }

    pub fn candidates(&self, pt: Pt, random: bool) -> impl IntoIterator<Item = u8> {
        Candidates::new(self, pt, random)
    }

    pub fn validate(&self) -> Result<(), String> {
        let convert = |pt| {
            let it = Iter { pt, b: self };
            it.map(|(_, n)| n.unwrap_or(0))
        };

        // TODO: return more detailed error information

        // Check if each col, row, and block contains 1-9
        for i in 0..9 {
            self.validate_iter(convert(PtIter::col(Pt::new(i, 0).unwrap())))?;
            self.validate_iter(convert(PtIter::row(Pt::new(0, i).unwrap())))?;
            self.validate_iter(convert(PtIter::block(
                Pt::new(i % BLOCK_WIDTH, i / BLOCK_WIDTH).unwrap(),
            )))?;
        }
        Ok(())
    }

    fn validate_iter(&self, it: impl Iterator<Item = u8>) -> Result<(), String> {
        let mask = it.map(|n| 1u32 << n).fold(0, |a, b| a | b);
        if mask == 0b11_1111_1110 {
            Ok(())
        } else {
            Err("wrong answer".to_string())
        }

        // TODO: this can be changed to (mask == 0b1_1111_1110).then_some(()).ok_or(Err(...))
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

struct Candidates {
    a: [u8; 9],
    n: usize,
    i: usize,
}

impl Candidates {
    fn new(b: &Board, pt: Pt, random: bool) -> Self {
        // TODO: Compute bit flags of the grid instead of checking other grids every time.
        // Using bool array might be faster than bit operations.
        let mut a = [1u8, 2, 3, 4, 5, 6, 7, 8, 9];
        let mut n = 0;
        for i in 0..9 {
            a[n] = a[i];
            if b.placeable(pt, a[i] as u8) {
                n += 1;
            }
        }
        if random && n > 1 {
            a[..n].shuffle(&mut rand::thread_rng());
        }
        Self { a, n, i: 0 }
    }
}

impl Iterator for Candidates {
    type Item = u8;

    fn next(&mut self) -> Option<u8> {
        if self.i == self.n {
            return None;
        }
        let c = self.a[self.i];
        self.i += 1;
        Some(c)
    }
}

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

    #[test]
    fn candidates() {
        let b = Board::default();
        assert!((1..=9).eq(b.candidates(Pt::new(0, 0).unwrap(), false).into_iter()));
    }

    #[test]
    fn validate_default() {
        let b = Board::default();
        assert!(b.validate().is_err());
    }
}
