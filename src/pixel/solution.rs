use super::puzzle::Puzzle;

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
pub enum CellState
{
    Filled,
    Empty,
    Undecided
}
pub type Assignment = Vec<CellState>;

pub type Sequence = Vec<u8>;

#[derive(Clone)]
pub struct Solution
{
    pub assignment: Assignment,
    pub sequence: Sequence,
}

impl Solution
{
    pub fn new(puzzle: &Puzzle) -> Solution
    {
        let max_cell_per_row = puzzle.sheet.iter().map(|row| row.iter().max().unwrap());
        let max_cell = *max_cell_per_row.max().unwrap() as usize;
        Solution {
            assignment: vec![CellState::Undecided; max_cell + 1],
            sequence: Vec::new()
        }
    }
}
