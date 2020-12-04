/*

    Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.

    ...

*/

use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let before = Instant::now();

    let filename = "src/number.txt";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut total: u128 = 0;

    for line in reader.lines() {
        let line = line.unwrap();
        let chopped = &line[0..35];
        let n: u128 = chopped.parse::<u128>().unwrap();
        
        total += n;
    }

    let mut answer = total.to_string();

    if answer.len() > 10 {
        answer = String::from(answer)[0..10].to_string();
    }

    println!("Answer: {}", answer);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
