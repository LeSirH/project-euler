/*

    A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:

    1/2	= 	0.5
    1/3	= 	0.(3)
    1/4	= 	0.25
    1/5	= 	0.2
    1/6	= 	0.1(6)
    1/7	= 	0.(142857)
    1/8	= 	0.125
    1/9	= 	0.(1)
    1/10	= 	0.1
    Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.

    Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.

    Link: https://projecteuler.net/problem=26
    Run time: 442.69ms

*/

use std::time::Instant;
use std::collections::HashMap;

fn main() {
    display_instructions();

    let before = Instant::now();
    let solution: u16 = solve();

    println!("Answer: {}", solution);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn solve() -> u16 {
    // We already know 7 has a recurring cycle of 6.
    let mut answer = 7;
    let mut largest_recurring = 6;

    for n in (answer + 1)..1001 {
        let recurring_count = find_recurring(n);

        if recurring_count > largest_recurring {
            largest_recurring = recurring_count;
            answer = n;
        }
    }

    answer
}

// Returns the number of digits recurring.
// If there are none, it will return 0.
fn find_recurring(n: u16) -> u16 {
    let mut result = String::new();
    let mut remainder = 1 % n;
    let mut map = HashMap::new();

    // As long as it isn't terminating and doesn't repeat 
    while (remainder != 0) && (!map.contains_key(&remainder.to_string())) {
        // Keep track of remainder history to prevent repeats
        map.insert(remainder.to_string(), result.len());

        remainder = remainder * 10;
        let new_recurring_digit = ((remainder / n) as f32).floor();

        // Append the recurring digit
        result = format!("{}{}", result, new_recurring_digit.to_string());
        
        remainder = remainder % n;
    }

    // No recurring sequence
    if remainder == 0 {
        result = String::new();
    }

    result.len() as u16
}

fn display_instructions() {
    println!("\n#26: A unit fraction contains 1 in the numerator. The decimal representation of the unit fractions with denominators 2 to 10 are given:");
    println!("1/2	= 	0.5");
    println!("1/3	= 	0.(3)");
    println!("1/4	= 	0.25");
    println!("1/5	= 	0.2");
    println!("1/6	= 	0.1(6)");
    println!("1/7	= 	0.(142857)");
    println!("1/8	= 	0.125");
    println!("1/9	= 	0.(1)");
    println!("1/10	= 	0.1");
    println!("Where 0.1(6) means 0.166666..., and has a 1-digit recurring cycle. It can be seen that 1/7 has a 6-digit recurring cycle.");
    println!("Find the value of d < 1000 for which 1/d contains the longest recurring cycle in its decimal fraction part.");
    println!("\nLink: https://projecteuler.net/problem=26\n");
}
