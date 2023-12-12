#[derive(Clone, PartialEq)]
pub struct Position {
    pub row: usize,
    pub col: usize
}

impl Position {
    pub fn new(row: usize, col: usize) -> Position {
        Position {
            row,
            col
        }
    }
}