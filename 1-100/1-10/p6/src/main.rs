/*

    The sum of the squares of the first ten natural numbers is,
    12 + 22 + ... + 102 = 385
    The square of the sum of the first ten natural numbers is,
    (1 + 2 + ... + 10)2 = 552 = 3025
    Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.
    Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

    Run time: 352.90µs

*/

use std::time::Instant;

fn main() {
    println!("\nQ: The sum of the squares of the first ten natural numbers is,\n\n\t12 + 22 + ... + 102 = 385\n\nThe square of the sum of the first ten natural numbers is,\n\n\t(1 + 2 + ... + 10)2 = 552 = 3025\n\nHence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 − 385 = 2640.\nFind the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.\n");
    let before = Instant::now();
    let mut square_sum: i64 = 0;
    let mut sum_square: i64 = 0;
    
    for i in 0..101 {
        square_sum += i64::pow(i as i64, 2);
        sum_square += i as i64;
    }
    
    let ans = (i64::pow(sum_square, 2)) - square_sum;
    
    println!("Answer: {}", ans);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
