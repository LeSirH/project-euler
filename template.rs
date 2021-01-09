/*

    [Instructions]

    Link: https://projecteuler.net/problem=n
    Run time: 

*/

use std::time::Instant;

struct Solver {
    start_time: Instant,
    problem_num: u8
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            start_time: Instant::now(),
            problem_num: 0 // Update to current problem #
        }
    }

    pub fn display_instructions(&self) {
        println!("\n#{}: [Instructions]", self.problem_num);
        println!("\nLink: https://projecteuler.net/problem={}\n", self.problem_num);
    }
}

fn main() {
    let solver = Solver::new();
    solver.display_instructions();

    let before = Instant::now();
    let solution: u8 = solve();

    println!("Answer: {}", solution);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn solve() -> u8 {
    0
}
