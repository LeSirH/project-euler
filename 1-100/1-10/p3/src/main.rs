/*

    The prime factors of 13195 are 5, 7, 13 and 29.
    What is the largest prime factor of the number 600851475143?

*/

use std::time::Instant;

fn main() {
    println!("\nQ: The prime factors of 13195 are 5, 7, 13 and 29.\nWhat is the largest prime factor of the number 600851475143?\n");

    let before = Instant::now();
    let n: i64 = 600851475143;
    let mut i: i64 = 1;
    
    while i <= (n / 2) {
        i += 1;
        let factor: f64 = (n as f64) / (i as f64);
        
        if factor.fract() == 0.0 {
            if is_prime(factor as i64) {
                println!("Answer: {}", factor);
                break;
            }
        }
    }

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
