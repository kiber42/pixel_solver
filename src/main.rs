mod pixel;
use pixel::PUZZLE;
use pixel::Solver;

fn main() {
    let solver = Solver::new(&PUZZLE);
    PUZZLE.print(true);
    if let Some(solution) = solver.run()
    {
        for step in &solution.sequence
        {
            println!("Mark {} as {:?}", step, solution.assignment[*step as usize]);
        }
        println!("Puzzle solved.");
        PUZZLE.print_solution(&solution.assignment, false);
    }
    else { println!("Puzzle is impossible."); }
}
