const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const SIZE: usize = WIDTH * HEIGHT;

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

    pub fn get(&self, x: usize, y: usize) -> Option<u8> {
        if x >= WIDTH || y >= HEIGHT {
            return None;
        }

        let v = self.numbers[y * WIDTH + x];
        if v == 0 {
            None
        } else {
            Some(v)
        }
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

        for x in 0..9 {
            for y in 0..9 {
                let expected = n[y * HEIGHT + x];
                if let Some(v) = b.get(x, y) {
                    assert_eq!(n[y * HEIGHT + x], v);
                } else {
                    assert_eq!(expected, 0);
                }
            }
        }
    }

    #[test]
    fn get_out_of_bounds() {
        let b = Board::new(&[1; SIZE]).unwrap();

        assert_eq!(Some(1), b.get(0, 0));
        assert_eq!(None, b.get(WIDTH, 0));
        assert_eq!(None, b.get(1000, 0));
        assert_eq!(None, b.get(0, HEIGHT));
        assert_eq!(None, b.get(0, 1000));
    }
}
