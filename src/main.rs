struct Puzzle {
    sheet: [[u8; GRID]; GRID],
    col_counts: [u8; GRID],
    row_counts: [u8; GRID]
}

const GRID : usize = 15;
const PUZZLE : Puzzle = Puzzle {
    sheet : [
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
    col_counts : [8, 6, 10, 5, 4, 4, 7, 4, 7, 6, 6, 5, 10, 7, 2],
    row_counts : [5, 11, 12, 3, 9, 8, 4, 8, 6, 3, 2, 2, 5, 6, 7]
};

/*
const GRID : usize = 10;
const PUZZLE : Puzzle = Puzzle {
    sheet : [
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
const GRID : usize = 5;
const PUZZLE : Puzzle = Puzzle {
    sheet : [
        [ 0, 1, 2, 2, 3],
        [ 0, 1, 2, 4, 4],
        [ 5, 6, 6, 7, 7],
        [ 8, 9, 9,10,10],
        [ 8, 9,11,11,11]],
    col_counts : [2, 3, 2, 1, 2],
    row_counts : [1, 2, 2, 3, 2]
};
*/

#[derive(PartialEq)]
enum GameState
{
    Solved,
    Impossible,
    Undecided
}

#[derive(PartialEq)]
#[derive(Copy, Clone)]
enum CellState
{
    Filled,
    Empty,
    Undecided
}
type Assignment = Vec<CellState>;

#[derive(PartialEq)]
enum Target
{
    Row(usize),
    Col(usize)
}

use std::fmt;
impl fmt::Display for Target {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match &self
        {
            Target::Row(r) => write!(f, "row {}", r + 1),
            Target::Col(c) => write!(f, "column {}", c + 1),
        }
    }
}

fn main() {
    if PUZZLE.run_solver() { println!("Puzzle solved."); }
    else { println!("Puzzle is impossible."); }
}

impl Puzzle
{
    fn run_solver(&self) -> bool
    {
        if !self.validate()
        {
            panic!("Puzzle definition is invalid.");
        }
    
        let max_cell_per_row = self.sheet.iter().map(|row| row.iter().max().unwrap());
        let max_cell = *max_cell_per_row.max().unwrap() as usize;
        let assignment = vec![CellState::Undecided; max_cell + 1];
        self.solve(assignment)
    }

    fn solve(&self, mut assignment: Assignment) -> bool
    {
        self.improve(&mut assignment);
        println!("Deduced:");
        self.print(&assignment);

        match self.get_state(&assignment)
        {
            GameState::Solved => return true,
            GameState::Impossible => return false,
            _ => {}
        }

        if let Some((index,_)) = assignment.iter().enumerate().find(|(_,&x)| x == CellState::Undecided)
        {
            assignment[index] = CellState::Filled;
            println!("Setting {} to FILLED (guess)", index);
            self.print(&assignment); // for debugging
            if self.solve(assignment.to_vec()) { return true; }

            println!("Setting {} to EMPTY (alternative was ruled out)", index);
            assignment[index] = CellState::Empty;
            self.print(&assignment); // for debugging
            return self.solve(assignment);
        }
        panic!("Should not get here.");
    }

    fn improve(&self, assignment: &mut Assignment)
    {
        loop
        {
            // TODO: Instead of iterating over all rows and columns like this,
            // keep a list of rows and cols that were recently changed.
            let mut changed_any = false;
            for row_or_col in 0..GRID
            {
                if self.get_state(assignment) == GameState::Impossible { return; }                
                if self.improve_once(assignment, Target::Row(row_or_col)) { changed_any = true; }
                if self.improve_once(assignment, Target::Col(row_or_col)) { changed_any = true; }
            }
            if !changed_any { break; }
        }
    }

    fn improve_once(&self, assignment: &mut Assignment, row_or_col : Target) -> bool
    {
        // determine which cells are in this row or column and how many squares they cover
        use std::collections::HashMap;
        let mut cell_sizes_in_target : HashMap<u8, u8> = HashMap::new();
        for other in 0..GRID
        {
            let cell = match row_or_col
            {
                Target::Row(r) => self.sheet[r][other],
                Target::Col(c) => self.sheet[other][c]
            };
            *cell_sizes_in_target.entry(cell).or_insert(0) += 1;
        }

        let (min, max) = self.get_range(assignment, &row_or_col);
        let current = match row_or_col
        {
            Target::Row(r) => self.row_counts[r],
            Target::Col(c) => self.col_counts[c]
        };
        let mut to_fill = current - min;
        let mut to_clear = max - current;

        let mut changed_any = false;
        loop 
        {
            let mut changed = false;
            for (&index, &count) in cell_sizes_in_target.iter()
            {
                let index = index as usize;
                if assignment[index] != CellState::Undecided { continue; }
                if count > to_fill && count > to_clear { return changed_any; }
                if count > to_clear
                {
                    assignment[index] = CellState::Filled;
                    changed = true;
                    to_fill -= count;
//                    println!("Setting {} to FILLED (deduced)", index);
                }
                else if count > to_fill
                {
                    assignment[index] = CellState::Empty;
                    changed = true;
                    to_clear -= count;
//                    println!("Setting {} to EMPTY (deduced)", index);
                }
            }
            if !changed { break; }
            changed_any = true;
        }
        changed_any
    }

    fn validate(&self) -> bool
    {
        // TODO: check whether target counts are consistent
        true
    }

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
                CellState::Empty => { max -= 1; }
                CellState::Filled => { min += 1; }
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
                if min > *goal || max < *goal
                {
                    println!("Current state unsolvable: {}, target count {} is not in the possible range {} - {}",
                        target, goal, min, max);
                    return GameState::Impossible;
                }
                if min != max { decided = false; }
            }
        }
        if decided { GameState::Solved } else { GameState::Undecided }
    }

    fn print(&self, assignment: &Assignment)
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
                    CellState::Filled => print!("<>"),
                    CellState::Empty => print!("__"),
                    CellState::Undecided => print!("??")
                }
            }
            println!();
        }
    }

}
