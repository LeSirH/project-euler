/*

    The following iterative sequence is defined for the set of positive integers:

    n → n/2 (n is even)
    n → 3n + 1 (n is odd)

    Using the rule above and starting with 13, we generate the following sequence:

    13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1
    It can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1.

    Which starting number, under one million, produces the longest chain?

    NOTE: Once the chain starts the terms are allowed to go above one million.

    Run time: 4.66s

*/

use std::time::Instant;

fn main() {
    println!("\nQ: The following iterative sequence is defined for the set of positive integers:\n\n\tn → n/2 (n is even)\n\tn → 3n + 1 (n is odd)\n\nUsing the rule above and starting with 13, we generate the following sequence:\n\n\t13 → 40 → 20 → 10 → 5 → 16 → 8 → 4 → 2 → 1\n\nIt can be seen that this sequence (starting at 13 and finishing at 1) contains 10 terms. Although it has not been proved yet (Collatz Problem), it is thought that all starting numbers finish at 1\n\nWhich starting number, under one million, produces the longest chain?\n\nNOTE: Once the chain starts the terms are allowed to go above one million.\n");

    let before = Instant::now();
    let mut start = 999999;
    let mut largest = 0;

    while start > 10000 {
        let seq = sequence(start).len();

        if seq > largest {
            largest = seq;
        }

        start -= 2;
    }
    
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn sequence(temp: i64) -> Vec<i64> {
    let mut n: i64 = temp;
    let mut sequence = vec![temp];

    while n != 1 {
        if n % 2 == 0 {
            n /= 2;
        } else {
            n = (3 * n) + 1
        }

        sequence.push(n);
    }

    sequence
}
