/*

    A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
    Find the largest palindrome made from the product of two 3-digit numbers.

    Run time: 2.25s

*/

use std::time::Instant;

fn main() {
    println!("\nQ: A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.\nFind the largest palindrome made from the product of two 3-digit numbers.\n");

    let before = Instant::now();
    let mut max_product = 0;
    
    for i in (100..999).rev() {
        for j in (100..999).rev() {
            let product = j * i;
            if is_palindrome(product) && product > max_product {
                max_product = product;
                break;
            }
        }
    }
    
    println!("Answer: {}", max_product);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn is_palindrome(n: i64) -> bool {
    let n_str = n.to_string();
    let reversed = n_str.chars().rev().collect::<String>();

    if n_str == reversed {
        return true;
    }

    return false;
}
