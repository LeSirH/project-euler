/*

    Work out the first ten digits of the sum of the following one-hundred 50-digit numbers.

    ...

*/

fn main() {
    use std::time::Instant;
    let before = Instant::now();
    let mut total = 0;

    /*
        * Read number from number.txt
        * Split the total into an array of digits
        * For each 50-digit number, get each digit, and while counting the digit place, add it to the correct digit in the total array. If the total exceeds 10, add to the previous element in the list.
        * Join the total into a string after total is calculated
    */

    println!("Answer: {}", total);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
