use crate::conversion;
use crate::rewriter;
use rand::prelude::*;
use std::collections::HashMap;

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

pub fn sat(formula: &str) -> bool {
    let mut variables = Vec::new();
    for c in formula.chars() {
        if ('A'..='Z').contains(&c) && !variables.contains(&c) {
            variables.push(c);
        }
    }
    let max_value = 1 << variables.len();
    for i in 0..max_value {
        let bits: Vec<char> = (0..variables.len())
            .map(|idx| if (i >> idx) & 1 == 1 { '1' } else { '0' })
            .collect();
        let letter_to_index: HashMap<char, usize> = variables
            .iter()
            .enumerate()
            .map(|(idx, &ch)| (ch, idx))
            .collect();
        let replaced_formula = formula
            .chars()
            .map(|c| {
                if let Some(&idx) = letter_to_index.get(&c) {
                    bits[idx].to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>();
        let result = eval_formula(&replaced_formula);
        if result {
            return true;
        }
    }
    false
}

pub fn formula_equivalence(formula1: &str, formula2: &str) -> bool {
    let mut variables = Vec::new();
    for c in formula1.chars() {
        if ('A'..='Z').contains(&c) && !variables.contains(&c) {
            variables.push(c);
        }
    }
    for c in formula2.chars() {
        if ('A'..='Z').contains(&c) && !variables.contains(&c) {
            return false;
        }
    }
    let max_value = 1 << variables.len();
    for i in 0..max_value {
        let bits: Vec<char> = (0..variables.len())
            .map(|idx| if (i >> idx) & 1 == 1 { '1' } else { '0' })
            .collect();
        let letter_to_index: HashMap<char, usize> = variables
            .iter()
            .enumerate()
            .map(|(idx, &ch)| (ch, idx))
            .collect();

        let replaced_formula1 = formula1
            .chars()
            .map(|c| {
                if let Some(&idx) = letter_to_index.get(&c) {
                    bits[idx].to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>();
        let result1 = eval_formula(&replaced_formula1);

        let replaced_formula2 = formula2
            .chars()
            .map(|c| {
                if let Some(&idx) = letter_to_index.get(&c) {
                    bits[idx].to_string()
                } else {
                    c.to_string()
                }
            })
            .collect::<String>();
        let result2 = eval_formula(&replaced_formula2);
        if result1 != result2 {
            return false;
        }
    }
    return true;
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

pub fn negation_normal_form(formula: &str) -> String {
    let btree = conversion::rpn_to_btree(formula);
    let btree = rewriter::btree_to_fnn(&btree);
    conversion::btree_to_rpn(&btree)
}

pub fn conjunctive_normal_form(formula: &str) -> String {
    let btree = conversion::rpn_to_btree(formula);
    let btree = rewriter::btree_to_cnf(&btree);
    conversion::btree_to_rpn(&btree)
}
