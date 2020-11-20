/*

    The prime factors of 13195 are 5, 7, 13 and 29.
    What is the largest prime factor of the number 600851475143 ?

*/

fn main() {
    let n: i64 = 600851475143;
    let mut i: i64 = 1;

    loop {
        i += 1;
        let factor: f64 = (n as f64) / (i as f64);
        
        if factor.fract() == 0.0 {
            if is_prime(factor) {
                println!("{}", factor);
                break;
            }
        }

        if i > (n / 2) {
            break;
        }
    }
}

fn is_prime(n: f64) -> bool {
    if n < 2f64 {
        return false;
    }

    for i in 2..(n.sqrt() as i64) {
        if (n as i64) % i == 0 {
            return false;
        }
    }

    return true;
}
