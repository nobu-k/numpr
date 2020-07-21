const WIDTH: usize = 9;
const HEIGHT: usize = 9;
const SIZE: usize = WIDTH * HEIGHT;

pub struct Board {
    numbers: [u8; SIZE], // TODO: use 4 bits per square
}

impl Board {
    pub fn new(n: &[u8]) -> Board {
        if n.len() != SIZE {
            panic!("slice must have length of 9: len = {}", n.len());
        }

        let mut b = [0; SIZE];
        b.copy_from_slice(n);
        return Board { numbers: b };
    }

    pub fn get(self, x: usize, y: usize) -> Option<u8> {
        if x > WIDTH || y > HEIGHT {
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
    #[should_panic]
    fn empty() {
        Board::new(&[]);
    }

    #[test]
    #[should_panic]
    fn too_short() {
        Board::new(&[0; SIZE - 1]);
    }

    #[test]
    #[should_panic]
    fn too_long() {
        Board::new(&[0; SIZE + 1]);
    }

    #[test]
    fn test_new() {
        let n: Vec<u8> = [0 as u8; SIZE]
            .iter()
            .enumerate()
            .map(|(i, _)| (i * 2) as u8)
            .collect();

        let b = Board::new(&n);
        assert_eq!(b.numbers.len(), SIZE);
        assert!(b.numbers.iter().eq(n.iter()));
    }
}
