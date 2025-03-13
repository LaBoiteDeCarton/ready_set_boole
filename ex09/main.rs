use boolean_algebra::set::*;
use boolean_algebra::*;

fn main() {
    let formula = formula::generate_symbolic_rpn(200);
    let mut operands: Vec<char> = Vec::new();
    for instruction in formula.chars() {
        if ('A'..='Z').contains(&instruction) && !operands.contains(&instruction) {
            operands.push(instruction);
        }
    }
    let sets = set::generate_sets(26, 6);
    println!("{:?}", formula);
    println!("{:?}", sets);
    let result_eval_set = eval_set(&formula, &sets);
    println!("{:?}", result_eval_set);
}
