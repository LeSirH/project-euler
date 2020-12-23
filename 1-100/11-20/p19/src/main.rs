/*

    You are given the following information, but you may prefer to do some research for yourself.

    - 1 Jan 1900 was a Monday.
    - Thirty days has September,
      April, June and November.
      All the rest have thirty-one,
      Saving February alone,
      Which has twenty-eight, rain or shine.
      And on leap years, twenty-nine.
    - A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.

    How many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?

    Run time: 8.73ms
    
*/

use std::time::Instant;

fn main() {
    println!("\nQ: You are given the following information, but you may prefer to do some research for yourself.\n\n- 1 Jan 1900 was a Monday.\n- Thirty days has September,\n  April, June and November.\n  All the rest have thirty-one,\n  Saving February alone,\n  Which has twenty-eight, rain or shine.\n  And on leap years, twenty-nine.\n- A leap year occurs on any year evenly divisible by 4, but not on a century unless it is divisible by 400.\n\nHow many Sundays fell on the first of the month during the twentieth century (1 Jan 1901 to 31 Dec 2000)?\n");

    let before = Instant::now();

    let mut days: i128 = 2;
    let mut total = 0;
    let mut days_per_month: Vec<i8> = vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];

    for year in 0..101 {
        if (year % 4 == 0) & (year != 100) {
            days_per_month[1] = 29;
        }

        for day_count in &days_per_month {
            if days % 7 == 1 {
                total += 1;
            }
            
            days += *day_count as i128;
        }

        if year == 0 {
            total = 0;
        }
    }

    println!("Answer: {}", total);
    println!("Elapsed time: {:.2?}", before.elapsed());
}
