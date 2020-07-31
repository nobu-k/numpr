use crate::pt::Pt;

// TODO: replace this raw error implementation with thiserror to see if there's
// any additional overhead.

#[derive(Debug, PartialEq, Eq)]
pub enum NumprError {
    IndexOutOfBounds(InvalidIndex),
    InvalidBoardLength(usize),
    InvalidValue(Pt, u8),
    WrongAnswer,
}

pub type NumprResult<T> = std::result::Result<T, NumprError>;
use NumprError::*;

impl NumprError {
    pub fn index_out_of_bounds<T>(x: usize, y: usize) -> NumprResult<T> {
        Err(IndexOutOfBounds(InvalidIndex { x, y }))
    }

    pub fn invalid_board_length<T>(len: usize) -> NumprResult<T> {
        Err(InvalidBoardLength(len))
    }

    pub fn invalid_value<T>(pt: Pt, n: u8) -> NumprResult<T> {
        Err(InvalidValue(pt, n))
    }

    pub fn wrong_answer<T>() -> NumprResult<T> {
        Err(WrongAnswer)
    }
}

impl std::fmt::Display for NumprError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            IndexOutOfBounds(i) => write!(f, "index out of bounds: ({}, {})", i.x, i.y),
            InvalidBoardLength(len) => write!(f, "slice must have length of 81: len = {}", *len),
            InvalidValue(pt, n) => write!(f, "invalid value at ({}, {}): {}", pt.x(), pt.y(), *n),
            WrongAnswer => write!(f, "wrong answer"), // TODO: more details
        }
    }
}

impl From<NumprError> for String {
    fn from(e: NumprError) -> String {
        format!("{}", e)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct InvalidIndex {
    x: usize,
    y: usize,
}
