/*

    The Fibonacci sequence is defined by the recurrence relation:

    Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.
    Hence the first 12 terms will be:

    F1 = 1
    F2 = 1
    F3 = 2
    F4 = 3
    F5 = 5
    F6 = 8
    F7 = 13
    F8 = 21
    F9 = 34
    F10 = 55
    F11 = 89
    F12 = 144
    The 12th term, F12, is the first term to contain three digits.

    What is the index of the first term in the Fibonacci sequence to contain 1000 digits?

    Link: https://projecteuler.net/problem=25
    Run time: 1.06ms

*/

use std::time::Instant;

fn main() {
    display_instructions();

    let before = Instant::now();
    let solution: u32 = solve();

    println!("Answer: {}", solution);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn solve() -> u32 {
    let mut index = 12;
    let mut count = 0; // number of times +5 has been added to the index
    let mut _digits = 3;

    while _digits < 1000 {
        if count == 4 {
            index += 4;
            count = 0;
            _digits += 1;
            continue;
        }

        index += 5;
        count += 1;
        _digits += 1;
    }

    index - 16
}

fn display_instructions() {
    println!("#25: The Fibonacci sequence is defined by the recurrence relation:");
    println!("Fn = Fn−1 + Fn−2, where F1 = 1 and F2 = 1.");
    println!("Hence the first 12 terms will be:");
    println!("F1 = 1, F2 = 1, F3 = 2, F4 = 3, F5 = 5, F6 = 8, F7 = 13, F8 = 21, F9 = 34, F10 = 55, F11 = 89, F12 = 144");
    println!("The 12th term, F12, is the first term to contain three digits.");
    println!("What is the index of the first term in the Fibonacci sequence to contain 1000 digits?");
    println!("\nLink: https://projecteuler.net/problem=25\n");
}
