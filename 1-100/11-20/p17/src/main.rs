/*

    If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.
    If all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?

    NOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of "and" when writing out numbers is in compliance with British usage.

*/

// I don't want to talk about this mess.

use std::time::Instant;

fn main() {
    println!("\nQ: If the numbers 1 to 5 are written out in words: one, two, three, four, five, then there are 3 + 3 + 5 + 4 + 4 = 19 letters used in total.\nIf all the numbers from 1 to 1000 (one thousand) inclusive were written out in words, how many letters would be used?\n\nNOTE: Do not count spaces or hyphens. For example, 342 (three hundred and forty-two) contains 23 letters and 115 (one hundred and fifteen) contains 20 letters. The use of \"and\" when writing out numbers is in compliance with British usage.\n");

    let before = Instant::now();

    let singles: Vec<&str> = vec!["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let teens: Vec<&str> = vec!["ten", "eleven", "twelve", "thirteen",  "fourteen", "fifteen", "sixteen",  "seventeen", "eighteen", "nineteen"];
    let doubles: Vec<&str> = vec!["twenty", "thirty", "forty",  "fifty", "sixty", "seventy",  "eighty", "ninety"];
    let triples: Vec<&str> = vec!["onehundred", "twohundred", "threehundred", "fourhundred", "fivehundred", "sixhundred", "sevenhundred", "eighthundred", "ninehundred"];

    // solve

    let single_digit_sum = count(singles);
    let double_digit_sum = (count(doubles) * 10) + count(teens) + (single_digit_sum * 8);
    let triple_digit_sum = (count(triples) * 100) + ((double_digit_sum + single_digit_sum) * 9) + 2673;

    println!("Answer: {}", single_digit_sum + double_digit_sum + triple_digit_sum + 11);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn count(arr: Vec<&str>) -> i64 {
    let mut sum: i64 = 0;
    
    for num_str in arr {
        sum += num_str.len() as i64;
    }

    sum
}
