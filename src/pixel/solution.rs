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
        Solution {
            assignment: vec![CellState::Undecided; puzzle.num_cells() as usize],
            sequence: Vec::new()
        }
    }
}
