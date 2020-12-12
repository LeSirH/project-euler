/*

    Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.
    For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.
    What is the total of all the name scores in the file?

*/

use std::time::Instant;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    display_instructions();
    let before = Instant::now();

    let mut names = get_names("src/names.txt");
    let mut scores: Vec<u64> = vec![];

    // sort alphabetically
    names.sort_by(|a, b| a.to_lowercase().cmp(&b.to_lowercase()));

    for (i, name) in names.iter().enumerate() {
        let name = name.to_owned();
        let name_values: Vec<u64> = name.chars().map(|chr| {
            return ((chr as u8) - 64) as u64;
        }).collect();

        let score: u64 = name_values.iter().sum::<u64>() * ((i + 1) as u64);
        scores.push(score);
    }

    println!("Answer: {}", scores.iter().sum::<u64>());
    println!("Elapsed time: {:.2?}", before.elapsed());
}

fn display_instructions() {
    println!("\nQ: Using names.txt (right click and 'Save Link/Target As...'), a 46K text file containing over five-thousand first names, begin by sorting it into alphabetical order. Then working out the alphabetical value for each name, multiply this value by its alphabetical position in the list to obtain a name score.");
    println!("For example, when the list is sorted into alphabetical order, COLIN, which is worth 3 + 15 + 12 + 9 + 14 = 53, is the 938th name in the list. So, COLIN would obtain a score of 938 × 53 = 49714.");
    println!("What is the total of all the name scores in the file?\n");
}

fn get_names(filename: &str) -> Vec<String> {
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut names: Vec<String> = vec![];

    for line in reader.lines() {
        line.unwrap().split(",").for_each(|name| {
            names.push(name.to_string().replace("\"", ""))
        });
    }

    return names;
}
