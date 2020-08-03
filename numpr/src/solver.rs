use crate::board::Board;
use crate::error::NumprResult;

/// A trait for solving a number place puzzle.
pub trait Solver {
    /// Solves the given puzzle in a Board.
    ///
    /// `board` contains the puzzle to be solved. It doesn't have to be filled
    /// at all. Passing `true` to `random` generates a random answer when the
    /// puzzle has multiple answers.
    ///
    /// It returns a completed board. The return value should be valid but
    /// call [`Board::validate`](struct.Board.html#method.validate) to make sure
    /// if it's the right answer.
    ///
    /// It returns [`NumprError`](enum.NumprError) when an error happens.
    ///
    /// # Examples
    ///
    /// ```
    /// use numpr::{Solver, solvers, Board};
    /// let b = solvers::NaiveSolver::new()
    ///     .solve(&Board::default(), true)
    ///     .unwrap();
    /// b.validate().unwrap();
    /// ```
    fn solve(self, board: &Board, random: bool) -> NumprResult<Board>;
}
