/*

    { Problem Instructions }

*/

use std::time::Instant;

fn main() {
    display_instructions();
    let before = Instant::now();

    // solve

    println!("Answer: ");
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn display_instructions() {
    println!("\nQ: { Problem Instructions }");
}
