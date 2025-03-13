use boolean_algebra::set::*;
use boolean_algebra::*;

fn main() {
    let powerset = powerset(vec![1, 2, 3, 4, 5, 6]);
    println!("{:?}", powerset);
    let formula = formula::generate_symbolic_rpn(200);
    let operands: Vec<char> = Vec::new();
    for instruction in formula.char() {
        if ['A'..'Z'].contains(instruction) && !operands.contains(instruction) {
            operands.push(instruction);
        }
    }

    let result_eval_set = eval_set(&formula, &powerset);
}
