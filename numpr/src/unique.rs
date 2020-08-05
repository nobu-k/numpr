use crate::board::Board;
use crate::error::NumprResult;
use crate::solver::Solver;

/// A trait to check if a Board has a unique answer.
pub trait UniquenessChecker {
    /// Checks if the given puzzle in a Board has exactly one answer.
    ///
    /// `board` contains the puzzle to be checked. `factory` is a function to
    /// create a new solver.
    ///
    /// It returns [`NumprError`](enum.NumprError) when the puzzle has multiple
    /// solutions or doesn't have any solution.
    ///
    /// # Examples
    ///
    /// ```
    /// use numpr::{Board, Pt, Solver, solvers, UniquenessChecker, uniques};
    ///
    /// # use numpr::NumprResult;
    /// # fn main() -> NumprResult<()> {
    /// let mut b = solvers::NaiveSolver::new()
    ///     .solve(&Board::default(), true)?;
    ///
    /// b.set(Pt::new(4, 4)?, 0);
    /// uniques::NaiveUniquenessChecker::new()
    ///     .check(&b, || solvers::NaiveSolver::new())?;
    /// # Ok(())
    /// # }
    /// ```
    fn check<S>(self, board: &Board, factory: impl Fn() -> S) -> NumprResult<()>
    where
        S: Solver;
}
