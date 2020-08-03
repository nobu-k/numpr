//! # numpr
//!
//! `numpr` provides utilities to solve, generate, and evaluate number place
//! puzzles.

mod board;
mod consts;
pub mod error;
mod generator;
pub mod generators;
mod placer;
pub mod placers;
pub mod pt;
mod solver;
pub mod solvers;
mod unique;
pub mod uniques;

pub use board::Board;
pub use consts::*;
pub use error::{NumprError, NumprResult};
pub use generator::Generator;
pub use placer::Placer;
pub use pt::{Pt, PtIter};
pub use solver::Solver;
pub use unique::UniquenessChecker;

// TODO: reorganize modules. For example, move placer.rs to placers/trait.rs.

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
