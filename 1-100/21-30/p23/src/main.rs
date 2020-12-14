/*

    A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
    A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
    As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
    Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

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
    let solutions: Vec<i32> = vec![];
    
    /*
        For the numbers from 12 to 28123:
        * Get all abundant numbers
        
        For each number from 24 to 28123 again:
        * Check if the number divided by 2 is an abundant number if the number is even
        * Check all other possibilities of being a sum of 2 abundant numbers
            * This means excluding abundant numbers greater than (n/2).ceil()
    */
    
    0
}

fn is_abundant(n: i32) -> bool {
    /*
        get all the divisors
        sum the divisors
        if the sum is greater than n, return true
        else return false
    */

    false
}

fn display_instructions() {
    println!("\nQ: A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.");
    println!("A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.");
    println!("As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.");
    println!("Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.\n");
}
