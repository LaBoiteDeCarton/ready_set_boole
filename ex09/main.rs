use boolean_algebra::set::*;
use boolean_algebra::*;

fn main() {
    let formula = formula::generate_symbolic_rpn(4);
    let mut operands: Vec<char> = Vec::new();
    for instruction in formula.chars() {
        if ('A'..='Z').contains(&instruction) && !operands.contains(&instruction) {
            operands.push(instruction);
        }
    }
    let sets = set::generate_sets(26, 6);
    println!("{:?}", formula);
    for (index, set) in sets.iter().enumerate() {
        println!("{}: {:?}", (index as u8 + 65) as char, set);
    }
    let result_eval_set = eval_set(&formula, &sets);
    println!("{:?}", result_eval_set);
}
