fn eval_formula(formula: &str) -> bool {
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

fn print_row(values: &Vec<char>) {
    print!("|");
    values.iter().for_each(|v| print!(" {} |", v));
    println!();
}

fn print_header(values: &Vec<char>) {
    print!("|");
    values.iter().for_each(|v| print!(" {} |", v));
    print!(" = |");
    println!();
    print!("|");
    values.iter().for_each(|_| print!("---|"));
    print!("---|");
    println!();
}

use std::collections::HashMap;

pub fn print_truth_table(formula: &str) {
    let mut variables = Vec::new();
    for c in formula.chars() {
        if c.is_ascii_uppercase() && !variables.contains(&c) {
            variables.push(c);
        }
    }
    print_header(&variables);
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
        let mut row = bits.clone();
        row.push(if result { '1' } else { '0' });
        print_row(&row);
    }
}
