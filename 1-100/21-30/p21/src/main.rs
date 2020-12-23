/*

    Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).
    If d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.

    For example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.

    Evaluate the sum of all the amicable numbers under 10000.

    Run time: 2.16s

*/

use std::time::Instant;

fn main() {
    println!("\nQ: Let d(n) be defined as the sum of proper divisors of n (numbers less than n which divide evenly into n).\nIf d(a) = b and d(b) = a, where a ≠ b, then a and b are an amicable pair and each of a and b are called amicable numbers.\n\nFor example, the proper divisors of 220 are 1, 2, 4, 5, 10, 11, 20, 22, 44, 55 and 110; therefore d(220) = 284. The proper divisors of 284 are 1, 2, 4, 71 and 142; so d(284) = 220.\n\nEvaluate the sum of all the amicable numbers under 10000.\n");

    let before = Instant::now();
    let mut amicable_nums: Vec<i32> = vec![];

    for i in 1..9999 {
        let d_1 = d(i);
        let d_2 = d(d_1);

        if d_2 == i && d_1 != i {
            amicable_nums.push(i);
        }
    }

    let ans: i32 = amicable_nums.iter().sum();

    println!("Answer: {}", ans);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn d(n: i32) -> i32 {
    let divisors: Vec<i32> = divisors(n);
    return divisors.iter().sum();
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
