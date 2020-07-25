use super::board::Board;

pub trait Solver {
    fn solve(self, board: &Board, random: bool) -> Result<Board, String>;
}

/* TODO: can this be abstracted like this?
pub struct Masks {
    // TODO: contain masks of grids to indicate which number can be placed.
    // TODO: implement this in a separate file.
}

pub trait PlacePicker {
    fn next(b: &Board, m: &mut Masks);
}

pub trait NumberPicker {
    fn for_each<F>(c: Candidates, func: F)
        where F: FnMut(u8) -> bool;
}

// TODO: move Picker to another file
// TODO: implement InOrderPicker and RandomPicker

pub fn solve<P>(b: &Board, pp: &impl PlacePicker, np: &impl NumberPicker) {

}
*/
