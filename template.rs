/*

    [Instructions]

    Link: https://projecteuler.net/problem=n
    Run time: 

*/

use std::time::Instant;

#[derive(Copy, Clone)]
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

    pub fn start_timer(mut self) {
        self.start_time = Instant::now();
    }
}

fn main() {
    let solver = Solver::new();
    solver.display_instructions();
    solver.start_timer();

    let solution = solve();

    println!("Answer: {}", solution);
    println!("Elapsed time: {:.2?}", solver.start_time.elapsed());
}

fn solve() -> u8 {
    0
}
