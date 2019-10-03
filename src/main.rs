const GRID : usize = 10;
const NUM_CELLS : usize = 66;
type Line = [u8; GRID];
type Sheet = [[u8; GRID]; GRID];
type State = [[bool; GRID]; GRID];
type Assignment = [i8; NUM_CELLS];

fn main() {
    let cells : Sheet = [
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
    let col_counts : Line = [7, 6, 6, 4, 3, 5, 7, 9, 8, 6];
    let row_counts : Line = [9, 9, 6, 4, 8, 5, 5, 2, 7, 6];

    let mut assign = [-1; NUM_CELLS];

    solve(cells, assign, col_counts, row_counts);

    println!("Done!");
//    println!("Puzzle solved? {}", is_solved(fill, col_counts, row_counts));
}

fn apply(assign: Assignment, sheet: Sheet) -> State
{
    let mut state : State = [[false; GRID]; GRID];
    for row in 0..GRID
    {
        for col in 0..GRID
        {
            state[row][col] = assign[sheet[row][col] as usize] > 0;
        }
    }
    return state;
}

fn apply_inverse(assign: Assignment, sheet: Sheet) -> State
{
    let mut state : State = [[false; GRID]; GRID];
    for row in 0..GRID
    {
        for col in 0..GRID
        {
            state[row][col] = assign[sheet[row][col] as usize] != 0;
        }
    }
    return state;
}

fn solve(cells: Sheet, assign: Assignment, cols: Line, rows: Line) -> bool
{
	let assign = improve(cells, assign, cols, rows);
    let fill = apply(assign, cells);
    if is_solved(fill, cols, rows)
    {
        print_solution(fill);
        let assign : Vec<bool> = assign.iter().map(|x| x > &0).collect();
        println!("Assignment: {:?}", assign);
        return true;
    }
    if is_too_much(fill, cols, rows)
    {
        return false;
    }
    let fill_inverse = apply_inverse(assign, cells);
    if !is_still_possible(fill_inverse, cols, rows)
    {
        return false;
    }
//    let cell_max = cells.iter().map(|col| col.iter().max().unwrap()).max().unwrap();
//    for cell in 0..=*cell_max as usize
    for cell in 0..NUM_CELLS
    {
        if assign[cell] == -1
        {
            let mut assign = assign;
            assign[cell] = 1;
            let result = solve(cells, assign, cols, rows);
            if result { return true; }
            assign[cell] = 0;
            let result = solve(cells, assign, cols, rows);
            if result { return true; }
        }
    }
    return false;
}

fn improve(cells: Sheet, assign: Assignment, cols: Line, rows: Line) -> Assignment
{
// TODO
	return assign;
}

fn is_solved(fill: State, cols: Line, rows: Line) -> bool
{
    for a in 0..GRID
    {
        let mut r_count = 0;
        let mut c_count = 0;
        for b in 0..GRID
        {
            if fill[a][b] { r_count += 1; }
            if fill[b][a] { c_count += 1; }
        }
        if r_count != rows[a] { return false; }
        if c_count != cols[a] { return false; }
    }
    return true;
}

fn is_too_much(fill: State, cols: Line, rows: Line) -> bool
{
    for a in 0..GRID
    {
        let mut r_count = 0;
        let mut c_count = 0;
        for b in 0..GRID
        {
            if fill[a][b] { r_count += 1; }
            if fill[b][a] { c_count += 1; }
        }
        if r_count > rows[a] { return true; }
        if c_count > cols[a] { return true; }
    }
    return false;
}

fn is_still_possible(fill: State, cols: Line, rows: Line) -> bool
{
    for a in 0..GRID
    {
        let mut r_count = 0;
        let mut c_count = 0;
        for b in 0..GRID
        {
            if fill[a][b] { r_count += 1; }
            if fill[b][a] { c_count += 1; }
        }
        if r_count < rows[a] { return false; }
        if c_count < cols[a] { return false; }
    }
    return true;
}

fn print_solution(fill: State)
{
    for r in 0..GRID
    {
        for c in 0..GRID
        {
            if fill[r][c] { print!("<>"); }
            else { print!("__"); }
        }
        println!();
    }
}