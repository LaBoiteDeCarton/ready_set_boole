use super::formula::eval_formula;
use std::collections::HashMap;

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

pub fn print_truth_table(formula: &str) {
    let mut variables = Vec::new();
    for c in formula.chars() {
        if ('A'..='Z').contains(&c) && !variables.contains(&c) {
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
