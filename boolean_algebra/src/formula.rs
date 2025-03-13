use rand::prelude::*;

pub fn eval_formula(formula: &str) -> bool {
    let mut instructions = Vec::new();

    for next_instruction in formula.chars() {
        match next_instruction {
            '0' => instructions.push(false),
            '1' => instructions.push(true),
            '!' => {
                let a = instructions.pop().expect("! instruction missing operand");
                instructions.push(!a);
            }
            '&' => {
                let a = instructions
                    .pop()
                    .expect("& instruction missing left operand");
                let b = instructions
                    .pop()
                    .expect("& instruction missing right operand");
                instructions.push(b && a);
            }
            '|' => {
                let a = instructions
                    .pop()
                    .expect("| instruction missing left operand");
                let b = instructions
                    .pop()
                    .expect("| instruction missing right operand");
                instructions.push(b || a);
            }
            'ˆ' => {
                let a = instructions
                    .pop()
                    .expect("ˆ instruction missing left operand");
                let b = instructions
                    .pop()
                    .expect("ˆ instruction missing right operand");
                instructions.push(a ^ b);
            }
            '>' => {
                let a = instructions
                    .pop()
                    .expect("> instruction missing left operand");
                let b = instructions
                    .pop()
                    .expect("> instruction missing right operand");
                instructions.push(!b || a);
            }
            '=' => {
                let a = instructions
                    .pop()
                    .expect("= instruction missing left operand");
                let b = instructions
                    .pop()
                    .expect("= instruction missing right operand");
                instructions.push(a == b);
            }
            _ => panic!("Invalid character in formula"),
        }
    }

    let result = instructions
        .pop()
        .expect("Formula did not result in a single boolean value");
    if instructions.is_empty() {
        result
    } else {
        panic!("Formula did not result in a single boolean value");
    }
}

pub fn generate_symbolic_rpn(max_length: u8) -> String {
    let mut result: String = String::new();
    let mut rng = rand::rng();
    let operators: Vec<char> = vec!['&', '|', 'ˆ', '>', '=', '!'];
    let operands: Vec<char> = ('A'..='Z').collect();
    let mut instructions_to_do = 1;
    while instructions_to_do > 0 {
        let is_operator: bool =
            instructions_to_do + result.len() < max_length as usize && rng.random_bool(0.5);
        if is_operator {
            let operator: char = operators[rng.random_range(0..operators.len())];
            result.push(operator);
            instructions_to_do += if operator == '!' { 0 } else { 1 };
        } else {
            let operand: char = operands[rng.random_range(0..operands.len())];
            result.push(operand);
            instructions_to_do -= 1;
        }
    }
    result.chars().rev().collect()
}

pub fn generate_evaluated_rpn(max_length: u8) -> String {
    let mut result: String = String::new();
    let mut rng = rand::rng();
    let operators: Vec<char> = vec!['&', '|', 'ˆ', '>', '=', '!'];
    let operands: Vec<char> = vec!['0', '1'];
    let mut instructions_to_do = 1;
    while instructions_to_do > 0 {
        let is_operator: bool =
            instructions_to_do + result.len() < max_length as usize && rng.random_bool(0.5);
        // let is_operator: bool = rng.random_bool(0.5);
        if is_operator {
            let operator: char = operators[rng.random_range(0..operators.len())];
            result.push(operator);
            instructions_to_do += if operator == '!' { 0 } else { 1 };
        } else {
            let operand: char = operands[rng.random_range(0..operands.len())];
            result.push(operand);
            instructions_to_do -= 1;
        }
    }
    result.chars().rev().collect()
}
