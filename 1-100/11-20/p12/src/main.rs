/*

    The sequence of triangle numbers is generated by adding the natural numbers. So the 7th triangle number would be 1 + 2 + 3 + 4 + 5 + 6 + 7 = 28. The first ten terms would be:
    1, 3, 6, 10, 15, 21, 28, 36, 45, 55, ...
    Let us list the factors of the first seven triangle numbers:

    1: 1
    3: 1,3
    6: 1,2,3,6
    10: 1,2,5,10
    15: 1,3,5,15
    21: 1,3,7,21
    28: 1,2,4,7,14,28

    We can see that 28 is the first triangle number to have over five divisors.
    What is the value of the first triangle number to have over five hundred divisors?

*/

fn main() {
    use std::time::Instant;
    let before = Instant::now();
    let mut n = 1;

    for x in 2..1000000 {
        n += x;
        if divisors(n) >= 500 {
            println!("{}", n);
            break;
        }
    }

    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn divisors(n: i64) -> i64 {
    let mut factors = 0;

    for i in 1..((n as f64).sqrt() as i64 + 1) {
        if n % i == 0 {
            factors += 2;
        }
    }

    return factors;
}
