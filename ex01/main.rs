use rand::prelude::*;
mod functions;

fn main() {
    let mut rng = rand::rng();
    let mut errors = 0;
    let mut nb_of_tests = 1000;
    println!("Starting multiplier test!");
    while nb_of_tests > 0 {
        let a = rng.random_range(0..10000000);
        let b = rng.random_range(0..10000000);
        let result = functions::multiplier(a, b);
        let (compare_result, overflowed) = a.overflowing_mul(b);
        if !overflowed {
            nb_of_tests -= 1;
            if compare_result != result {
                errors += 1;
                println!("💔 {} + {} = {} ?= {}", a, b, result, compare_result);
            } else {
                println!("💚 {} + {} = {} ?= {}", a, b, result, compare_result);
            }
        }
    }
    println!("{} errors found", errors);
}
