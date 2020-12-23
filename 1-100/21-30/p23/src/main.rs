/*

    A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.
    A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.
    As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.
    Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.

    Run time: 252.31s
*/

use std::time::Instant;

fn main() {
    display_instructions();

    let before = Instant::now();
    let solution: i32 = solve();

    println!("Answer: {}", solution);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn solve() -> i32 {
    let mut sum = 0;
    let abundant_nums: Vec<i32> = find_abundants();

    // Check if n cannot be written as the sum of two abundant numbers
    for n in 1..20162 {
        if (n % 2 == 0) & is_abundant(n / 2) {
            continue;
        }
 
        // Get a list of all abundant numbers below n
        let mut small_abundants: Vec<i32> = vec![];
        for j in &abundant_nums {
            if (j > &n) | (j == &n) {
                break;
            }

            small_abundants.push(*j);
        }

        // Check if any possible sum of abundants equals n
        let mut _solved = false;
        for i in &small_abundants {
            if _solved == true {
                break;
            }

            for j in &small_abundants {
                if i == j {
                    continue;
                }

                if i + j == n {
                    _solved = true;
                    break;
                }
            }
        }

        if _solved == false {
            sum += n;
        }
    }

    sum
}

fn find_abundants() -> Vec<i32> {
    let mut abundant_nums: Vec<i32> = vec![];

    // Get a list of all abundant numbers.
    for i in 12..28124 {
        if is_abundant(i) {
            abundant_nums.push(i);
        }
    }

    abundant_nums
}

fn divisors(n: i32) -> Vec<i32> {
    let mut divisors: Vec<i32> = vec![];

    if n == 1 {
        return vec![1];
    }

    for i in 1..((n / 2) + 1) {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    divisors
}

fn is_abundant(n: i32) -> bool {
    let divisors: Vec<i32> = divisors(n);
    let divisor_sum: i32 = divisors.iter().sum();

    // println!("N={}, S={} {}", n, divisor_sum, (divisor_sum > n));

    if divisor_sum > n {
        return true;
    }

    false
}

fn display_instructions() {
    println!("\nQ: A perfect number is a number for which the sum of its proper divisors is exactly equal to the number. For example, the sum of the proper divisors of 28 would be 1 + 2 + 4 + 7 + 14 = 28, which means that 28 is a perfect number.");
    println!("A number n is called deficient if the sum of its proper divisors is less than n and it is called abundant if this sum exceeds n.");
    println!("As 12 is the smallest abundant number, 1 + 2 + 3 + 4 + 6 = 16, the smallest number that can be written as the sum of two abundant numbers is 24. By mathematical analysis, it can be shown that all integers greater than 28123 can be written as the sum of two abundant numbers. However, this upper limit cannot be reduced any further by analysis even though it is known that the greatest number that cannot be expressed as the sum of two abundant numbers is less than this limit.");
    println!("Find the sum of all the positive integers which cannot be written as the sum of two abundant numbers.\n");
}
