/*

    Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.

    https://projecteuler.net/project/images/p015.png

    How many such routes are there through a 20×20 grid?

*/

use std::time::Instant;

fn main() {
    println!("\nQ: Starting in the top left corner of a 2×2 grid, and only being able to move to the right and down, there are exactly 6 routes to the bottom right corner.\n\n\thttps://projecteuler.net/project/images/p015.png\n\nHow many such routes are there through a 20×20 grid?\n");

    let before = Instant::now();

    // solve
    println!("Answer: {}", middle(pascal_triangle(40)));

    println!("Answer: ");
    println!("Elapsed time: {:.2?}", before.elapsed());
}

// Find the middle number of an array
fn middle(arr: Vec<i64>) -> i64 {
    return arr[arr.len() / 2];
}

fn pascal_triangle(n: i64) -> Vec<i64> {
    let mut pascal: Vec<i64> = vec![1];
    let mut new_pascal: Vec<i64> = vec![1];
    let mut count: i64 = 1;

    while count <= n {
        for num in 0..(pascal.len() - 1) {
            new_pascal.push(pascal[num] + pascal[num + 1]);
        }

        new_pascal.push(1);
        pascal = new_pascal;
        new_pascal = vec![1];
        count += 1;
    }

    pascal
}
