/*

    Euler discovered the remarkable quadratic formula:
    
        n^2 + n + 41

    It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39. However, when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisilbe by 41, and certainly when n = 41, 41^2 + 41 + 41 is clearly divisble by 41.

    The incredible formula n^2 - 79 + 1601 was discovered, which produces 80 primes for the consecutive values 0 <= n <= 79. The product of the coefficients, -79 and 1601, is -126479

    Considering quadratics of the form:

        n^2 + an  + b, where |a| < 1000 and |b| <= 1000

        where |n| is the modulus/absolute value of n
        e.g. |11| = 11 and |-4| = 4
    
    Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.

    Link: https://projecteuler.net/problem=27
    Run time: 

*/

use std::time::Instant;

#[derive(Copy, Clone)]
struct Solver {
    start_time: Instant,
    problem_num: u8
}

impl Solver {
    pub fn new() -> Solver {
        Solver {
            start_time: Instant::now(),
            problem_num: 27
        }
    }

    pub fn display_instructions(&self) {
        println!("\n#{}: Euler discovered the remarkable quadratic formula:\n", self.problem_num);
        println!("\tn^2 + n + 41\n");
        println!("It turns out that the formula will produce 40 primes for the consecutive integer values 0 <= n <= 39. However, when n = 40, 40^2 + 40 + 41 = 40(40 + 1) + 41 is divisilbe by 41, and certainly when n = 41, 41^2 + 41 + 41 is clearly divisble by 41.\n");
        println!("The incredible formula n^2 - 79 + 1601 was discovered, which produces 80 primes for the consecutive values 0 <= n <= 79. The product of the coefficients, -79 and 1601, is -126479\n");
        println!("Considering quadratics of the form:\n");
        println!("\tn^2 + an  + b, where |a| < 1000 and |b| <= 1000\n");
        println!("\twhere |n| is the modulus/absolute value of n");
        println!("\te.g. |11| = 11 and |-4| = 4\n");
        println!("Find the product of the coefficients, a and b, for the quadratic expression that produces the maximum number of primes for consecutive values of n, starting with n = 0.\n");
        println!("\nLink: https://projecteuler.net/problem={}\n", self.problem_num);
    }

    pub fn start_timer(mut self) {
        self.start_time = Instant::now();
    }
}

fn main() {
    let solver = Solver::new();
    solver.display_instructions();
    solver.start_timer();

    let solution = solve();

    println!("Answer: {}", solution);
    println!("Elapsed time: {:.2?}", solver.start_time.elapsed());
}

fn solve() -> u8 {
    0
}
