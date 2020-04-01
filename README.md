# Pixel puzzle solver

A solver for logic puzzles that was written as a first exercise in Rust using VS Code.

Get, compile, and run the example puzzle with
```
git clone https://github.com/kiber42/pixel_solver
cd pixel_solver
cargo build
target/debug/pixel_solver
```

A puzzle definition consists of a square grid size, cell definitions, and the number of "pixels" that need to be filled for each row and for each column.
Cells consist of one or more pixels and are defined in a 2D-array by assigning them consecutive indices starting at 0.

```
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
```

This defines a puzzle of size 5x5 (`GRID`), with 12 cells (number and shapes defined by `PUZZLE`).  In this example the cells are between 1 and 3 pixels in size.  In the solved state, each cell has to be either completely filled or completely blank.  As an additional constraint, a total of 2 pixels needs to be filled in the first column, 3 pixels in the second column, 2 in the third column and so on (`col_counts`), and similar constraints apply to the fill-counts of each row (`row_counts`).

For simplicity, the puzzles are hard-coded at the moment.  The file `puzzle.rs` contains two further example puzzles, all but one puzzle are commented out.  Comment out the first one and add your own puzzle, or comment in one of the other examples to try them out, then build and run the project.
