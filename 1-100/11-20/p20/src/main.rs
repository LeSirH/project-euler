/*

    n! means n × (n − 1) × ... × 3 × 2 × 1

    For example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,
    and the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.

    Find the sum of the digits in the number 100!

    Run time: 11.02ms

*/

extern crate num_bigint;
use std::time::Instant;
use num_bigint::{BigInt, Sign, ToBigInt};

fn main() {
    println!("\nQ: n! means n × (n − 1) × ... × 3 × 2 × 1\n\nFor example, 10! = 10 × 9 × ... × 3 × 2 × 1 = 3628800,\nand the sum of the digits in the number 10! is 3 + 6 + 2 + 8 + 8 + 0 + 0 = 27.\n\nFind the sum of the digits in the number 100!\n");

    let before = Instant::now();
    let n = factorial(100.to_bigint().unwrap());
    let answer: i16 = sum_digits(n);
    
    println!("Answer: {}", answer);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn sum_digits(n: BigInt) -> i16 {
    let num_string: String = n.to_string();
    let mut sum = 0;

    for i in num_string.chars() {
        sum += i.to_string().parse::<i16>().unwrap();
    }

    sum
}

fn factorial(num: BigInt) -> BigInt {
    if num == BigInt::new(Sign::Plus, vec![1]) || num == BigInt::new(Sign::Plus, vec![0]) {
        return BigInt::new(Sign::Plus, vec![1]);
    }

    return factorial(&num - 1) * num
}
