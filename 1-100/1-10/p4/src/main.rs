/*

    A palindromic number reads the same both ways. The largest palindrome made from the product of two 2-digit numbers is 9009 = 91 × 99.
    Find the largest palindrome made from the product of two 3-digit numbers.

*/

fn main() {
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

    println!("{}", max_product);
}

fn is_palindrome(n: i64) -> bool {
    let n_str = n.to_string();
    let reversed = n_str.chars().rev().collect::<String>();

    if n_str == reversed {
        return true;
    }

    return false;
}
