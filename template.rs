/*

    { Problem Instructions }

*/

use std::time::Instant;

fn main() {
    display_instructions();

    let before = Instant::now();
    let solution: u8 = solve();

    println!("Answer: {}", solution);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn solve() -> u8 {
    0
}

fn display_instructions() {
    println!("\nQ: { Problem Instructions }");
}
