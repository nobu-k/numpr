mod board;
mod solver;
mod solvers;

pub use board::Board;
pub use solver::Solver;
pub use solvers::NaiveSolver;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
