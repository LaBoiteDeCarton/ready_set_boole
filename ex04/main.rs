use boolean_algebra::formula::*;
use boolean_algebra::truth_table::*;

fn basic_formula_truth_table() {
    println!("Formula is : {}", "AB&");
    print_truth_table("AB&");
    println!();
    println!("Formula is : {}", "AB|");
    print_truth_table("AB|");
    println!();
    println!("Formula is : {}", "AB>");
    print_truth_table("AB>");
    println!();
    println!("Formula is : {}", "AB=");
    print_truth_table("AB=");
    println!();
    println!("Formula is : {}", "ABˆ");
    print_truth_table("ABˆ");
    println!();
    println!("Formula is : {}", "H!");
    print_truth_table("H!");
    println!();
    println!("Formula is : {}", "AAˆ");
    print_truth_table("AAˆ");
    println!();
}

fn random_formula_truth_table() {
    let rand_rpn = generate_symbolic_rpn(10);
    println!("Formula is : {}", rand_rpn);
    print_truth_table(rand_rpn.as_str());
    println!();
}

fn main() {
    basic_formula_truth_table();
    for _ in 0..10 {
        random_formula_truth_table();
    }
}
