/*

    A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:

        012   021   102   120   201   210

    What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?

    Link: https://projecteuler.net/problem=24
    Run time: 17.9s

*/

use std::time::Instant;

fn main() {
    display_instructions();

    let before = Instant::now();
    let solution: String = solve();

    println!("Answer: {}", solution);
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn solve() -> String {
    let digits: Vec<usize> = (0..10).collect();
    let permutations: Vec<String> = gen_permutations(&digits);
    let one_millionth_perm: &String = &permutations[999999];

    return one_millionth_perm.clone();
}

fn gen_permutations(nums: &Vec<usize>) -> Vec<String> {    
    if nums.len() == 2 {
        return vec![
            format!("{}{}", nums[0], nums[1]), 
            format!("{}{}", nums[1], nums[0])
        ];
    }
    
    let mut permutations: Vec<String> = vec![];

    for (i, _) in nums.iter().enumerate() {
        let mut temp_nums: Vec<usize> = nums.clone();
        let temp_num = temp_nums.remove(i);

        let temp_permutations = gen_permutations(&temp_nums);
        for n in temp_permutations {
            permutations.push(format!("{}{}", temp_num, n));
        }
    }

    return permutations;
}

fn display_instructions() {
    println!("\n#24: A permutation is an ordered arrangement of objects. For example, 3124 is one possible permutation of the digits 1, 2, 3 and 4. If all of the permutations are listed numerically or alphabetically, we call it lexicographic order. The lexicographic permutations of 0, 1 and 2 are:");
    println!("\n012   021   102   120   201   210\n");
    println!("What is the millionth lexicographic permutation of the digits 0, 1, 2, 3, 4, 5, 6, 7, 8 and 9?");
    println!("\nLink: https://projecteuler.net/problem=24\n");
}
