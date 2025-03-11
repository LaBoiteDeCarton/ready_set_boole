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
