use super::board::Board;

pub trait Solver {
    fn solve(b: &mut Board) -> Result<(), String>;
}
