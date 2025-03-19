use boolean_algebra::formula::*;

fn main() {
    println!("{} should be false", eval_formula("10&"));
    println!("{} should be true", eval_formula("10|"));
    println!("{} should be true", eval_formula("11>"));
    println!("{} should be false", eval_formula("10="));
    println!("{} should be true", eval_formula("1011||="));
    println!("{} should be true", eval_formula("1010&1&&&"));
    println!("{} should be true", eval_formula("1"));
    println!("{} should be false", eval_formula("0"));
}
