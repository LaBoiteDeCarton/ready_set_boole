use rand::prelude::*;
mod functions;

fn main() {
    let mut rng = rand::rng();
    let mut errors = 0;
    println!("Starting adder test!");
    for _ in 0..1000 {
        let a = rng.random::<u32>();
        let b = rng.random::<u32>();
        let result = functions::adder(a, b);
        let (compare_result, overflowed) = a.overflowing_add(b);
        if !overflowed {
            if compare_result != result {
                errors += 1;
                println!("💔 {} + {} = {} ?= {}", a, b, result, compare_result);
            } else {
                println!("💚 {} + {} = {} == {}", a, b, result, compare_result);
            }
        }
    }
    println!("{} errors found", errors);
}
