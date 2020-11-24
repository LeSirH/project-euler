/*

    2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
    What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

*/

use std::time::Instant;

fn main() {
    let before = Instant::now();
    let mut n = 2520;
    let nums = [
        1, 2, 3, 4, 5, 
        6, 7, 8, 9, 10, 
        11, 12, 13, 14, 15, 
        16, 17, 18, 19, 20
    ];
        
    loop {
        if (n % 2 == 0) & (n % 3 == 0) & deep_check(n, nums) {
            println!("{}", n);
            break;
        }
        
        n += 2520;
    }

    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn deep_check(n: i64, nums: [i64; 20]) -> bool {
    for num in nums.iter() {
        if n % num != 0 {
            return false;
        }
    }

    return true;
}
