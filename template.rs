/*

    [Instructions]

    Link: https://projecteuler.net/problem=n
    Run time: 

*/

use std::time::Instant;

struct Solver {
    start_time: Instant
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            start_time: Instant::now()
        }
    }

    pub fn display_instructions(&self) {
        println!("\n#n: [Instructions]");
        println!("\nLink: https://projecteuler.net/problem=n\n");
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
