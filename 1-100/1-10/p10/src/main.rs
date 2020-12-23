/*

    The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.
    Find the sum of all the primes below two million.

    Run time: 2.23s

*/

fn main() {
    println!("\nQ: The sum of the primes below 10 is 2 + 3 + 5 + 7 = 17.\nFind the sum of all the primes below two million.\n");

    use std::time::Instant;
    let before = Instant::now();

    // solve
    let target: i64 = 2000000;
    let mut primes: Vec<i64> = Vec::new();
    let mut n: i64 = 1;
    let mut sum: i64 = 0;

    while n < target {
        if is_prime(n) {
            primes.push(n);
        }

        n += 2;
    }

    for prime in primes {
        sum += prime;
    }

    println!("Answer: {}", sum);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn is_prime(n: i64) -> bool {
    if n == 1 {
        return false;
    }

    let mut i: i64 = 2;
    while (i * i) <= n {
        if n % i == 0 {
            return false;
        }

        i += 1;
    }

    return true;
}
