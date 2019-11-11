#[derive(Clone, Copy)]
struct Puzzle {
    sheet: [[u8; GRID]; GRID],
    col_counts: [u8; GRID],
    row_counts: [u8; GRID]
}

const GRID: usize = 15;
const PUZZLE: Puzzle = Puzzle {
    sheet: [
        [ 0, 1, 2, 2, 3, 3, 3, 3, 3, 4, 4, 5, 5, 6, 7],
        [ 0, 1, 2, 8, 8, 3, 9,10,11,12,12,13,14,15,16],
        [17,18,18,19,19, 9, 9, 9,20,21,22,13,14,16,16],
        [17,23,23,24,24,25, 9,26,27,28,29,29,14,16,30],
        [17,23,31,31,31,25,32,26,33,33,33,29,34,34,30],
        [35,35,36,36,37,38,32,32,39,39,39,34,34,40,30],
        [35,41,42,43,37,44,44,45,45,46,47,48,49,40,50],
        [51,41,52,43,53,53,54,54,55,55,56,56,56,57,50],
        [51,58,59,60,60,53,54,54,61,61,61,62,62,63,50],
        [64,65,66,67,68,68,68,69,70,71,71,72,73,63,74],
        [64,65,75,75,75,76,76,69,70,70,77,77,73,63,74],
        [78,79,80,80,80,81,81,69,82,83,83,84,85,85,74],
        [78,79,79,80,80,81,86,86,82,82,83,84,85,85,87],
        [78,78,79,88,89,86,86,86,90,82,90,91,91,92,87],
        [78,78,79,93,93,93,94,94,90,90,90,91,91,91,87]],
    col_counts: [8, 6, 10, 5, 4, 4, 7, 4, 7, 6, 6, 5, 10, 7, 2],
    row_counts: [5, 11, 12, 3, 9, 8, 4, 8, 6, 3, 2, 2, 5, 6, 7]
};

/*
const GRID: usize = 10;
const PUZZLE: Puzzle = Puzzle {
    sheet: [
        [ 0, 1, 1, 2, 2, 3, 3, 4, 4, 5],
        [ 0, 0, 1, 6, 7, 7, 8, 9,10, 5],
        [11,11,12,13,13,14,15,15,16,17],
        [18,19,19,20,21,22,23,24,24,25],
        [18,26,26,27,28,28,23,29,30,30],
        [18,31,32,27,33,34,35,29,36,37],
        [18,38,32,39,40,41,42,43,44,45],
        [46,47,48,48,49,50,50,51,45,45],
        [46,52,48,53,54,55,56,51,57,58],
        [59,60,61,62,54,63,63,64,64,65]],
    col_counts: [7, 6, 6, 4, 3, 5, 7, 9, 8, 6],
    row_counts: [9, 9, 6, 4, 8, 5, 5, 2, 7, 6]
};
*/

/*
const GRID: usize = 5;
const PUZZLE: Puzzle = Puzzle {
    sheet: [
        [ 0, 1, 2, 2, 3],
        [ 0, 1, 2, 4, 4],
        [ 5, 6, 6, 7, 7],
        [ 8, 9, 9,10,10],
        [ 8, 9,11,11,11]],
    col_counts: [2, 3, 2, 1, 2],
    row_counts: [1, 2, 2, 3, 2]
};
*/

#[derive(PartialEq)]
#[derive(Copy, Clone)]
#[derive(Debug)]
enum CellState
{
    Filled,
    Empty,
    Undecided
}
type Assignment = Vec<CellState>;

type Sequence = Vec<u8>;

#[derive(Clone)]
struct Solution
{
    assignment: Assignment,
    sequence: Sequence,
}

// holds pre-computed structures used throughout the solving process
use std::collections::HashMap;
struct Solver
{
    puzzle: Puzzle,
    cell_sizes_in_rows: [HashMap<u8, u8>; GRID],
    cell_sizes_in_cols: [HashMap<u8, u8>; GRID],
}

fn main() {
    let solver = Solver::new(&PUZZLE);
    if let Some(solution) = solver.run()
    {
        for step in &solution.sequence
        {
            println!("Mark {} as {:?}", step, solution.assignment[*step as usize]);
        }
        println!("Puzzle solved.");
    }
    else { println!("Puzzle is impossible."); }
}

#[derive(PartialEq)]
enum GameState
{
    Solved,
    Impossible,
    Undecided
}

#[derive(PartialEq)]
enum Target
{
    Row(usize),
    Col(usize)
}

impl Puzzle
{
    fn validate(&self) -> bool
    {
        let col_counts = self.col_counts.iter();
        let row_counts = self.row_counts.iter();
        let cols_sum : usize = col_counts.map(|&u| u as usize).sum();
        let rows_sum : usize = row_counts.map(|&u| u as usize).sum();
        cols_sum == rows_sum &&
        self.col_counts.iter().max().unwrap() <= &(GRID as u8) &&
        self.row_counts.iter().max().unwrap() <= &(GRID as u8)
    }
}

impl Solution
{
    fn new(puzzle: &Puzzle) -> Solution
    {
        let max_cell_per_row = puzzle.sheet.iter().map(|row| row.iter().max().unwrap());
        let max_cell = *max_cell_per_row.max().unwrap() as usize;
        Solution {
            assignment: vec![CellState::Undecided; max_cell + 1],
            sequence: Vec::new()
        }
    }
}

impl Solver
{
    fn new(puzzle: &Puzzle) -> Solver
    {
        if !puzzle.validate()
        {
            panic!("Puzzle definition is invalid.");
        }

        // determine which cells are in this row or column and how many squares they cover
        let mut cell_sizes_in_rows : [HashMap<u8, u8>; GRID] = Default::default();
        let mut cell_sizes_in_cols : [HashMap<u8, u8>; GRID] = Default::default();
        for row in 0..GRID
        {
            for col in 0..GRID
            {
                let cell = puzzle.sheet[row][col];
                *cell_sizes_in_rows[row].entry(cell).or_insert(0) += 1;
                *cell_sizes_in_cols[col].entry(cell).or_insert(0) += 1;
            }
        }

        Solver
        {
            puzzle: *puzzle,
            cell_sizes_in_rows: cell_sizes_in_rows,
            cell_sizes_in_cols: cell_sizes_in_cols,
        }
    }

    fn run(&self) -> Option<Solution>
    {
        self.recurse(&Solution::new(&self.puzzle))
    }

    fn recurse(&self, partial: &Solution) -> Option<Solution>
    {
        let mut updated = partial.clone();
        self.improve(&mut updated);
        println!("Deduced:");
        self.puzzle.print(&updated.assignment, true);

        match self.puzzle.get_state(&updated.assignment)
        {
            GameState::Solved => return Some(updated),
            GameState::Impossible => return None,
            _ => {}
        }

        if let Some((index,_)) = updated.assignment.iter().enumerate().find(|(_,&x)| x == CellState::Undecided)
        {
            updated.sequence.push(index as u8);
            for state in &[CellState::Filled, CellState::Empty]
            {
                println!("Assigning {} as {:?}", index, state);
                updated.assignment[index] = *state;
                let result = self.recurse(&updated);
                if result.is_some() { return result; }
            }
            return None;
        }
        panic!("Should not get here.");
    }

    fn improve(&self, partial: &mut Solution)
    {
        loop
        {
            // TODO: Instead of iterating over all rows and columns like this,
            // keep a list of rows and cols that were recently changed.
            let mut changed = false;
            for row_or_col in 0..GRID
            {
                changed |= self.improve_once(partial, Target::Row(row_or_col));
                changed |= self.improve_once(partial, Target::Col(row_or_col));
            }
            if !changed { break; }
        }
    }

    fn improve_once(&self, partial: &mut Solution, row_or_col: Target) -> bool
    {
        let (min, max) = self.puzzle.get_range(&partial.assignment, &row_or_col);
        let goal = match row_or_col
        {
            Target::Row(r) => self.puzzle.row_counts[r],
            Target::Col(c) => self.puzzle.col_counts[c]
        };
        if goal < min || goal > max { return false; }
        let mut to_fill = goal - min;
        let mut to_clear = max - goal;

        let mut changed = false;
        let cell_sizes_in_target = match row_or_col
        {
            Target::Row(r) => &self.cell_sizes_in_rows[r],
            Target::Col(c) => &self.cell_sizes_in_cols[c]
        };
        loop
        {
            let mut seq = Vec::new();
            for (&index, &count) in cell_sizes_in_target
            {
                let ref mut assign = partial.assignment[index as usize];
                if *assign != CellState::Undecided { continue; }
                if count > to_fill && count > to_clear { return false; }
                if count > to_clear
                {
                    *assign = CellState::Filled;
                    seq.push(index);
                    to_fill -= count;
                }
                else if count > to_fill
                {
                    *assign = CellState::Empty;
                    seq.push(index);
                    to_clear -= count;
                }
            }
            if seq.is_empty() { break; }
            changed = true;
            partial.sequence.extend(seq);
        }
        changed
    }
}

impl Puzzle
{
    fn get_range(&self, assignment: &Assignment, row_or_col: &Target) -> (u8, u8)
    {
        let mut min = 0;
        let mut max = GRID as u8;
        for other in 0..GRID
        {   
            let cell = match row_or_col
            {
                Target::Row(r) => self.sheet[*r][other],
                Target::Col(c) => self.sheet[other][*c]
            };
            match assignment[cell as usize]
            {
                CellState::Empty => max -= 1,
                CellState::Filled => min += 1,
                CellState::Undecided => {}
            }
        }
        (min, max)
    }

    fn get_state(&self, assignment: &Assignment) -> GameState
    {
        let mut decided = true;
        for index in 0..GRID
        {
            for (target, goal) in &[(Target::Row(index), self.row_counts[index]),
                                    (Target::Col(index), self.col_counts[index])]
            {
                let (min, max) = self.get_range(assignment, &target);
                if min > *goal || max < *goal { return GameState::Impossible; }
                if min != max { decided = false; }
            }
        }
        if decided { GameState::Solved } else { GameState::Undecided }
    }

    fn print(&self, assignment: &Assignment, borders: bool)
    {
        if borders { self.print_with_borders(assignment); }
        else { self.print_plain(assignment); }
    }

    fn print_with_borders(&self, assignment: &Assignment)
    {
        print!("   ");
        for c in 0..GRID
        {
            print!("{:3}", self.col_counts[c]);
        }
        println!();
        println!();

        for r in 0..GRID
        {
            print!("{:3} ", self.row_counts[r]);
            for c in 0..GRID
            {
                let cell = self.sheet[r][c];
                match assignment[cell as usize] {
                    CellState::Filled => print!("██"),
                    CellState::Empty => print!("  "),
                    CellState::Undecided => print!("{:2}", cell)
                }
                if c < GRID - 1
                {
                    let right = self.sheet[r][c+1];
                    print!("{}", divider_horizontal(cell, right));
                }
            }
            println!();
            print!("    ");
            if r < GRID - 1
            {
                for c in 0..GRID
                {
                    let cell = self.sheet[r][c];
                    let below = self.sheet[r+1][c];
                    print!("{}", divider_vertical(cell, below));
                    if c < GRID - 1
                    {
                        let right = self.sheet[r][c+1];
                        let diag = self.sheet[r+1][c+1];
                        print!("{}", divider_junction(cell, right, below, diag));
                    }
                }
            }
            println!();
        }
    }

    fn print_plain(&self, assignment: &Assignment)
    {
        print!("   ");
        for c in 0..GRID
        {
            print!("{:2}", self.col_counts[c]);
        }
        println!();

        for r in 0..GRID
        {
            print!("{:2} ", self.row_counts[r]);
            for c in 0..GRID
            {
                match assignment[self.sheet[r][c] as usize] {
                    CellState::Filled => print!("██"),
                    CellState::Empty => print!("  "),
                    CellState::Undecided => print!("??")
                }
            }
            println!();
        }
    }
}

fn divider_horizontal(cell: u8, right: u8) -> char
{
    if cell == right { ' ' } else { '│' }
}

fn divider_vertical(cell: u8, below: u8) -> String
{
    (if cell == below { "  " } else { "──" }).to_string()
}

fn divider_junction(cell: u8, right: u8, below: u8, diag: u8) -> char
{
    const DIVIDERS : [char; 16] = [' ', ' ', ' ', '│', ' ', '┘', '┐', '┤', ' ', '└', '┌', '├', '─', '┴', '┬', '┼'];
    let top    = if cell != right { 1 } else { 0 };
    let bottom = if below != diag { 2 } else { 0 };
    let left   = if cell != below { 4 } else { 0 };
    let right  = if right != diag { 8 } else { 0 };
    let index = top + bottom + left + right;
    DIVIDERS[index]
}
