type Targets = [u8; GRID];
type Sheet = [[u8; GRID]; GRID];
type Assignment = [i8; NUM_CELLS];

const GRID : usize = 15;
const NUM_CELLS : usize = 95;
const PUZZLE : Sheet = [
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
[78,78,79,93,93,93,94,94,90,90,90,91,91,91,87]];
const PUZZLE_COL_COUNTS : Targets = [8, 6, 10, 5, 4, 4, 7, 4, 7, 6, 6, 5, 10, 7, 2];
const PUZZLE_ROW_COUNTS : Targets = [5, 11, 12, 3, 9, 8, 4, 8, 6, 3, 2, 2, 5, 6, 7];

/*
const GRID : usize = 10;
const NUM_CELLS : usize = 66;
const PUZZLE : Sheet = [
[ 0, 1, 1, 2, 2, 3, 3, 4, 4, 5],
[ 0, 0, 1, 6, 7, 7, 8, 9,10, 5],
[11,11,12,13,13,14,15,15,16,17],
[18,19,19,20,21,22,23,24,24,25],
[18,26,26,27,28,28,23,29,30,30],
[18,31,32,27,33,34,35,29,36,37],
[18,38,32,39,40,41,42,43,44,45],
[46,47,48,48,49,50,50,51,45,45],
[46,52,48,53,54,55,56,51,57,58],
[59,60,61,62,54,63,63,64,64,65]];
const PUZZLE_COL_COUNTS : Targets = [7, 6, 6, 4, 3, 5, 7, 9, 8, 6];
const PUZZLE_ROW_COUNTS : Targets = [9, 9, 6, 4, 8, 5, 5, 2, 7, 6];
*/

/*
const GRID : usize = 5;
const NUM_CELLS : usize = 12;
const PUZZLE : Sheet = [
[ 0, 1, 2, 2, 3],
[ 0, 1, 2, 4, 4],
[ 5, 6, 6, 7, 7],
[ 8, 9, 9,10,10],
[ 8, 9,11,11,11]];
const PUZZLE_COL_COUNTS : Targets = [2, 3, 2, 1, 2];
const PUZZLE_ROW_COUNTS : Targets = [1, 2, 2, 3, 2];
*/

use std::collections::HashMap;

fn main() {
    let result = solve(PUZZLE, [-1; NUM_CELLS], PUZZLE_COL_COUNTS, PUZZLE_ROW_COUNTS);
    if result { println!("Puzzle solved."); }
    else { println!("Puzzle is impossible."); }
    println!("Done!");
}

fn solve(sheet: Sheet, assignment: Assignment, cols: Targets, rows: Targets) -> bool
{
	let mut assignment = improve(sheet, assignment, cols, rows);
    print_solution(sheet, assignment); // for debugging

    match get_state(sheet, assignment, cols, rows)
    {
        GameState::Solved => return true,
        GameState::Impossible => return false,
        _ => {}
    }

    if let Some(cell) = (0..NUM_CELLS).find(|&x| assignment[x] == -1)
    {
        assignment[cell] = 1;
        println!("Setting {} to FILLED (guess)", cell);
        print_solution(sheet, assignment); // for debugging
        let result = solve(sheet, assignment, cols, rows);
        if result { return true; }
        println!("Setting {} to EMPTY (alternative was ruled out)", cell);
        assignment[cell] = 0;
        return solve(sheet, assignment, cols, rows);
    }    
    return true;
}

fn improve(sheet: Sheet, assignment: Assignment, col_targets: Targets, row_targets: Targets) -> Assignment
{
    let mut assignment = assignment;
    loop
    {
        let mut changed_any = false;
        for r in 0..GRID
        {
            let (rows_min, rows_max, _, _) = get_min_max_line_counts(sheet, assignment);
//            println!("Row {}: Target = {}, current range = {} - {}", r, row_targets[r], rows_min[r], rows_max[r]);
//            print_solution(sheet, assignment);
            if get_state(sheet, assignment, col_targets, row_targets) == GameState::Impossible { return assignment; }
            let mut to_fill = row_targets[r] - rows_min[r];
            let mut to_clear = rows_max[r] - row_targets[r];
//            println!("To fill: {}, to clear: {}", to_fill, to_clear);
            // determine which cells are in this row and how many squares they cover
            let mut cell_sizes_in_row : HashMap<u8, u8> = HashMap::new();
            for c in 0..GRID
            {
                let cell = sheet[r][c];
                *cell_sizes_in_row.entry(cell).or_insert(0) += 1;
            }
            loop 
            {
                let mut changed = false;
                for (index, count) in cell_sizes_in_row.iter()
                {
                    let index = *index as usize;
                    // only work on cells that have not been assigned "empty" or "filled" yet
                    if assignment[index] != -1 { continue; }
                    if count > &to_fill && count > &to_clear { return assignment; }
                    if count > &to_clear
                    {
                        assignment[index] = 1;
                        changed_any = true;
                        changed = true;
                        to_fill -= count;
                        println!("Setting {} to FILLED (deduced)", index);
                    }
                    else if count > &to_fill
                    {
                        assignment[index] = 0;
                        changed_any = true;
                        changed = true;
                        to_clear -= count;
                        println!("Setting {} to EMPTY (deduced)", index);
                    }
                }
                if !changed { break; }
            }
        }
        if !changed_any { break; }
    }
    // TODO implement transpose function and re-use above code
    loop
    {
        let mut changed_any = false;
        for c in 0..GRID
        {
            let (_, _, cols_min, cols_max) = get_min_max_line_counts(sheet, assignment);
//            println!("Column {}: Target = {}, current range = {} - {}", c, col_targets[c], cols_min[c], cols_max[c]);
//            print_solution(sheet, assignment);
            if get_state(sheet, assignment, col_targets, row_targets) == GameState::Impossible { return assignment; }
            let mut to_fill = col_targets[c] - cols_min[c];
            let mut to_clear = cols_max[c] - col_targets[c];
//            println!("To fill: {}, to clear: {}", to_fill, to_clear);
            // determine which cells are in this columns and how many squares they cover
            let mut cell_sizes_in_col : HashMap<u8, u8> = HashMap::new();
            for r in 0..GRID
            {
                let cell = sheet[r][c];
                *cell_sizes_in_col.entry(cell).or_insert(0) += 1;
            }
            loop 
            {
                let mut changed = false;
                for (index, count) in cell_sizes_in_col.iter()
                {
                    let index = *index as usize;
                    // only work on cells that have not been assigned "empty" or "filled" yet
                    if assignment[index] != -1 { continue; }
                    if count > &to_fill && count > &to_clear { return assignment; }
                    if count > &to_clear
                    {
                        assignment[index] = 1;
                        changed_any = true;
                        changed = true;
                        to_fill -= count;
                        println!("Setting {} to FILLED (deduced)", index);
                    }
                    else if count > &to_fill
                    {
                        assignment[index] = 0;
                        changed_any = true;
                        changed = true;
                        to_clear -= count;
                        println!("Setting {} to EMPTY (deduced)", index);
                    }
                }
                if !changed { break; }
            }
        }
        if !changed_any { break; }
    }
	return assignment;
}

fn get_min_max_line_counts(sheet: Sheet, assignment: Assignment) -> (Targets, Targets, Targets, Targets)
{
    let mut rows_min = [0; GRID];
    let mut cols_min = [0; GRID];
    let mut rows_max = [GRID as u8; GRID];
    let mut cols_max = [GRID as u8; GRID];
    for row in 0..GRID
    {
        for col in 0..GRID
        {   
            match assignment[sheet[row][col] as usize]
            {
                0 => { rows_max[row] -= 1; cols_max[col] -= 1; },
                1 => { rows_min[row] += 1; cols_min[col] += 1; },
                _ => {}
            }
        }
    }
    return (rows_min, rows_max, cols_min, cols_max);
}

#[derive(PartialEq)]
enum GameState
{
    Solved,
    Impossible,
    Undecided
}

fn get_state(sheet: Sheet, assignment: Assignment, col_targets: Targets, row_targets: Targets) -> GameState
{
    let (r_min, r_max, c_min, c_max) = get_min_max_line_counts(sheet, assignment);
    for index in 0..GRID
    {
        if r_min[index] > row_targets[index] || r_max[index] < row_targets[index] ||
          c_min[index] > col_targets[index] || c_max[index] < col_targets[index]
        {
            println!("Current state unsolvable:");
            println!("  Col {}: Target is {}; possible range is {} - {}", index, col_targets[index], c_min[index], c_max[index]);
            println!("  Row {}: Target is {}; possible range is {} - {}", index, row_targets[index], r_min[index], r_max[index]);
            return GameState::Impossible;
        }
    }
    for index in 0..GRID
    {
        if r_min[index] != r_max[index] || c_min[index] != c_max[index] { return GameState::Undecided; }
    }
    return GameState::Solved;
}

fn print_solution(sheet: Sheet, assignment: Assignment)
{
    for r in 0..GRID
    {
        for c in 0..GRID
        {
            match assignment[sheet[r][c] as usize] {
                1 => print!("<>"),
                0 => print!("__"),
                _ => print!("??")
            }
        }
        println!();
    }
}
