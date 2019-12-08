use super::solver::Target;
use super::solution::Assignment;
use super::solution::CellState;

#[derive(Clone, Copy)]
pub struct Puzzle {
    pub sheet: [[u8; GRID]; GRID],
    pub col_counts: [u8; GRID],
    pub row_counts: [u8; GRID]
}

pub const GRID: usize = 15;
pub const PUZZLE: Puzzle = Puzzle {
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

impl Puzzle
{
    pub fn validate(&self) -> bool
    {
        let col_counts = self.col_counts.iter();
        let row_counts = self.row_counts.iter();
        let cols_sum : usize = col_counts.map(|&u| u as usize).sum();
        let rows_sum : usize = row_counts.map(|&u| u as usize).sum();
        cols_sum == rows_sum &&
        self.col_counts.iter().max().unwrap() <= &(GRID as u8) &&
        self.row_counts.iter().max().unwrap() <= &(GRID as u8)
    }

    pub fn get_range(&self, assignment: &Assignment, row_or_col: &Target) -> (u8, u8)
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

    pub fn is_decided(&self, assignment: &Assignment) -> Option<bool>
    {
        let mut solved = true;
        for index in 0..GRID
        {
            for (target, goal) in &[(Target::Row(index), self.row_counts[index]),
                                    (Target::Col(index), self.col_counts[index])]
            {
                let (min, max) = self.get_range(assignment, &target);
                if min > *goal || max < *goal { return Some(false); }
                if min != max { solved = false; }
            }
        }
        if solved { Some(true) } else { None }
    }

    pub fn print(&self, borders: bool)
    {
        self.do_print(None, borders);
    }

    pub fn print_solution(&self, assignment: &Assignment, borders: bool)
    {
        self.do_print(Some(assignment), borders);
    }

    fn do_print(&self, assignment: Option<&Assignment>, borders: bool)
    {
        let width = if borders { 3 } else { 2 };
        print!("   ");
        for c in 0..GRID
        {
            print!("{:1$}", self.col_counts[c], width);
        }
        println!();
        if borders { println!(); }

        for r in 0..GRID
        {
            print!("{:1$} ", self.row_counts[r], width);
            for c in 0..GRID
            {
                let cell = self.sheet[r][c];
                if assignment.is_some()
                {
                    match assignment.unwrap()[cell as usize] {
                        CellState::Filled => print!("██"),
                        CellState::Empty => print!("  "),
                        CellState::Undecided => print!("{:2}", cell)
                    };
                }
                else { print!("{:2}", cell); }
                if borders && c < GRID - 1
                {
                    let right = self.sheet[r][c+1];
                    print!("{}", divider_horizontal(cell, right));
                }
            }
            println!();
            if borders
            {
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
