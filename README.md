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
pub const GRID: usize = 5;
pub const PUZZLE: Puzzle = Puzzle {
    sheet: [
        [ 0, 1, 2, 3, 3],
        [ 0, 0, 2, 2, 3],
        [ 5, 5, 6, 4, 4],
        [ 7, 8, 6, 4, 9],
        [ 7, 8, 6,10,10]],
    col_counts: [3, 5, 0, 4, 4],
    row_counts: [4, 3, 4, 2, 3]
};
```

This defines a puzzle of size 5x5 (`GRID`), with 11 cells (number and shapes defined by `PUZZLE`).  In this example the cells are between 1 and 3 pixels in size.  In the solved state, each cell has to be either completely filled or completely blank.  As additional constraints, a total of 3 pixels needs to be filled in the first column, 5 pixels in the second column, none in the third column and so on (`col_counts`), and similar constraints apply to the fill-counts of each row (`row_counts`).

A more intuitive way to present the puzzle above is
```
     3  5  0  4  4

  4  0│ 1│ 2│ 3  3
      └──┤  └──┐  
  3  0  0│ 2  2│ 3
    ─────┼──┬──┴──
  4  5  5│ 6│ 4  4
    ──┬──┤  │  ┌──
  2  7│ 8│ 6│ 4│ 9
      │  │  ├──┴──
  3  7│ 8│ 6│10 10
```

The solution for this small example is
```
    3 5 0 4 4
 4 ████  ████
 3 ████    ██
 4 ████  ████
 2   ██  ██  
 3   ██  ████
```

For simplicity, the puzzles are hard-coded at the moment.  The file `puzzle.rs` contains two further example puzzles, all but one puzzle are commented out.  Comment out the first one and add your own puzzle, or comment in one of the other examples to try them out, then build and run the project.
