/*

    215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.

    What is the sum of the digits of the number 2^1000?

    Run time: 17.24ms

*/

use std::time::Instant;

fn main() {
    println!("\nQ: 215 = 32768 and the sum of its digits is 3 + 2 + 7 + 6 + 8 = 26.\nWhat is the sum of the digits of the number 21000?\n");

    let before = Instant::now();
    let mut digits: Vec<i64> = vec![1];
    let mut answer = 0; 
    let mut carry = 0;

    for _ in 0..1000 {
        for i in (0..digits.len()).rev() {
            let x = carry + digits[i] * 2;
            carry = 0;

            if x > 9 && i != 0 {
                digits[i] = x % 10;
                carry = x / 10;
            } 
            else {
                digits[i] = x;
            }
        }

        if digits[0] > 9 {
            let t: i64 = digits[0] / 10;
            let x: i64 = digits[0] % 10;

            digits[0] = x;
            digits.insert(0, t);
        }
    }

    for i in digits.iter() {
        answer += i; 
    }

    println!("Answer: {}", answer);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
