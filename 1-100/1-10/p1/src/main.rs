/*

    If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
    Find the sum of all the multiples of 3 or 5 below 1000.

    Run time: 354.30µs

*/

use std::time::Instant;

fn main() {
    println!("\nQ: If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.\nFind the sum of all the multiples of 3 or 5 below 1000.\n");
    let before = Instant::now();

    // I had originally attempted using i16.
    let mut sum: u32 = 23;

    for value in 10..1000 {
        if (value % 3 == 0) || (value % 5 == 0) {
            sum += value;
        }
    }

    println!("Answer: {}", sum);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
