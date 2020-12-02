/*

    By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.
    What is the 10 001st prime number?

*/

use std::time::Instant;

fn main() {
    println!("\nQ: By listing the first six prime numbers: 2, 3, 5, 7, 11, and 13, we can see that the 6th prime is 13.\nWhat is the 10 001st prime number?\n");

    let before = Instant::now();
    let mut prime_count: i16 = 1;
    let mut n = 3;
    
    while prime_count < 10001 {
        if is_prime(n) {
            prime_count += 1;
        }
        
        n += 2;
    }
    
    println!("Answer: {}", n - 1);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }

    if n % 2 == 0 {
        return n == 2;
    }
    
    for i in 2..((n as f64).sqrt() as i64 + 1) {
        if n % i == 0 {
            return false;
        }
    }
    
    return true;
}
