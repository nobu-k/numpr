mod board;
mod consts;
mod generator;
mod generators;
mod placer;
mod placers;
mod pt;
mod solver;
mod solvers;
mod unique;
mod uniques;

pub use board::Board;
pub use consts::*;
pub use placer::Placer;
pub use placers::NaivePlacer;
pub use pt::*;
pub use solver::Solver;
pub use solvers::NaiveSolver;
pub use unique::UniquenessChecker;
pub use uniques::NaiveUniquenessChecker;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
