use super::puzzle::{Puzzle, GRID};
use super::solution::{CellState, Solution};

use std::collections::HashMap;

// holds pre-computed structures used throughout the solving process
pub struct Solver
{
    puzzle: Puzzle,
    cell_sizes_in_rows: [HashMap<u8, u8>; GRID],
    cell_sizes_in_cols: [HashMap<u8, u8>; GRID],
    affected_targets_by_cell: Vec<Vec<Target>>,
}

#[derive(PartialEq, Eq, Clone)]
pub enum Target
{
    Row(usize),
    Col(usize)
}

impl Solver
{
    pub fn new(puzzle: &Puzzle) -> Solver
    {
        if !puzzle.validate()
        {
            panic!("Puzzle definition is invalid.");
        }

        // determine which cells are in this row or column and how many squares they cover
        let mut cell_sizes_in_rows : [HashMap<u8, u8>; GRID] = Default::default();
        let mut cell_sizes_in_cols : [HashMap<u8, u8>; GRID] = Default::default();
        // also determine in which rows and columns each cell appears
        let mut affected_targets_by_cell = vec![Vec::new(); puzzle.num_cells() as usize];
        for row in 0..GRID
        {
            for col in 0..GRID
            {
                let cell = puzzle.sheet[row][col];
                *cell_sizes_in_rows[row].entry(cell).or_insert(0) += 1;
                *cell_sizes_in_cols[col].entry(cell).or_insert(0) += 1;
                let targets = &mut affected_targets_by_cell[cell as usize];
                targets.push(Target::Row(row));
                targets.push(Target::Row(col));
            }
        }

        Solver
        {
            puzzle: *puzzle,
            cell_sizes_in_rows: cell_sizes_in_rows,
            cell_sizes_in_cols: cell_sizes_in_cols,
            affected_targets_by_cell: affected_targets_by_cell,
        }
    }

    pub fn run(&self) -> Option<Solution>
    {
        self.recurse(&Solution::new(&self.puzzle))
    }

    fn recurse(&self, partial: &Solution) -> Option<Solution>
    {
        let mut updated = partial.clone();
        let still_possible = self.improve(&mut updated);
        if !still_possible
        {
            return None;
        }

        if let Some(solved) = self.puzzle.is_decided(&updated.assignment)
        {
            return if solved { Some(updated) } else { None };
        }

        if let Some((index,_)) = updated.assignment.iter().enumerate().find(|(_,&x)| x == CellState::Undecided)
        {
            updated.sequence.push(index as u8);
            for state in &[CellState::Filled, CellState::Empty]
            {
                updated.assignment[index] = *state;
                let result = self.recurse(&updated);
                if result.is_some() { return result; }
            }
            return None;
        }
        panic!("Should not get here.");
    }

    fn improve(&self, partial: &mut Solution) -> bool
    {
        let mut targets = Vec::new();
        for row_or_col in 0..GRID
        {
            targets.push(Target::Row(row_or_col));
            targets.push(Target::Col(row_or_col));
        }
        while let Some(target) = targets.pop()
        {
            let is_possible = self.improve_once(partial, &target);
            match is_possible
            {
                Some(changed_targets) => targets.extend(changed_targets),
                None => return false
            }
            targets.retain(|x| *x != target);
        }
        true
    }

    fn improve_once(&self, partial: &mut Solution, row_or_col: &Target) -> Option<Vec<Target>>
    {
        let (min, max) = self.puzzle.get_range(&partial.assignment, &row_or_col);
        let goal = match *row_or_col
        {
            Target::Row(r) => self.puzzle.row_counts[r],
            Target::Col(c) => self.puzzle.col_counts[c]
        };
        if goal < min || goal > max { return None; }
        let mut to_fill = goal - min;
        let mut to_clear = max - goal;

        let mut changed = Vec::new();
        let cell_sizes_in_target = match *row_or_col
        {
            Target::Row(r) => &self.cell_sizes_in_rows[r],
            Target::Col(c) => &self.cell_sizes_in_cols[c]
        };
        loop
        {
            let mut seq = Vec::new();
            // some randomness is introduced here since a HashMap is used
            for (&index, &count) in cell_sizes_in_target
            {
                let ref mut assign = partial.assignment[index as usize];
                if *assign != CellState::Undecided { continue; }
                if count > to_fill && count > to_clear { return None; }
                if count > to_clear
                {
                    *assign = CellState::Filled;
                    to_fill -= count;
                }
                else if count > to_fill
                {
                    *assign = CellState::Empty;
                    to_clear -= count;
                }
                else { continue; }
                seq.push(index);
                changed.extend(self.affected_targets_by_cell[index as usize].clone());
            }
            if seq.is_empty() { break; }
            partial.sequence.extend(seq);
        }
        Some(changed)
    }
}
