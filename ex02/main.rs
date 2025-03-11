use rand::prelude::*;
mod functions;
use gray_codes::GrayCode32;

fn main() {
    let mut rng = rand::rng();
    let mut errors = 0;
    println!("Starting multiplier test!");
    for _ in 0..1000 {
        let number = rng.random::<u32>();
        let result = functions::gray_code(number);
        let compare_result = GrayCode32::from_index(number);
        if compare_result != result {
            errors += 1;
            println!(
                "💔 number {} greyCode {} ?= {}",
                number, result, compare_result
            );
        } else {
            println!(
                "💚 number {} greyCode {} ?= {}",
                number, result, compare_result
            );
        }
    }
    for i in 0..20 {
        let number = i;
        let result = functions::gray_code(number);
        let compare_result = GrayCode32::from_index(number);
        if compare_result != result {
            errors += 1;
            println!(
                "💔 number {} greyCode {} ?= {}",
                number, result, compare_result
            );
        } else {
            println!(
                "💚 number {} greyCode {} ?= {}",
                number, result, compare_result
            );
        }
    }
    println!("{} errors found", errors);
}
