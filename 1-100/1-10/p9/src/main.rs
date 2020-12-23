/*

    A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,
    a2 + b2 = c2
    For example, 32 + 42 = 9 + 16 = 25 = 52.
    There exists exactly one Pythagorean triplet for which a + b + c = 1000.
    Find the product abc.

    Run time: 15.62ms
    
*/

fn main() {
    println!("\nQ: A Pythagorean triplet is a set of three natural numbers, a < b < c, for which,\na2 + b2 = c2\nFor example, 32 + 42 = 9 + 16 = 25 = 52.\nThere exists exactly one Pythagorean triplet for which a + b + c = 1000.\nFind the product abc.\n");

    use std::time::Instant;
    let before = Instant::now();
    let mut solved: bool = false;

    for a in 1..1000 {
        if solved {
            break;
        }

        for b in 1..1000 {
            let c = 1000 - a - b;
            let is_triplet = i32::pow(a, 2) + i32::pow(b, 2) == i32::pow(c, 2);

            if is_triplet {
                println!("Answer: {}", a * b * c);
                solved = true;
                break;
            }
        }
    }

    println!("Elapsed time: {:.2?}", before.elapsed());
}
