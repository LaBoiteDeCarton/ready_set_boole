use super::btree::BinaryTree;

pub fn rpn_to_btree(formula: &str) -> BinaryTree<char> {
    let mut btree: Vec<BinaryTree<char>> = Vec::new();

    for next_instruction in formula.chars() {
        match next_instruction {
            'A'..='Z' | '0' | '1' => {
                let node = BinaryTree::new(next_instruction);
                btree.push(node);
            }
            '!' => {
                let a = btree.pop().expect("! instruction missing operand");
                let mut node = BinaryTree::new(next_instruction);
                node.set_left(a);
                btree.push(node);
            }
            '&' | '|' | 'ˆ' | '>' | '=' => {
                let right = btree.pop().expect(&format!(
                    "{} instruction missing left operand",
                    next_instruction
                ));
                let left = btree.pop().expect(&format!(
                    "{} instruction missing right operand",
                    next_instruction
                ));
                let mut node = BinaryTree::new(next_instruction);
                node.set_left(left);
                node.set_right(right);
                btree.push(node);
            }
            _ => panic!("Invalid character in formula"),
        }
    }
    let result = btree
        .pop()
        .expect("Formula did not result in a single boolean value");
    if btree.is_empty() {
        result
    } else {
        panic!("Formula did not result in a single boolean value");
    }
}

pub fn btree_to_rpn(btree: &BinaryTree<char>) -> String {
    let mut result = String::new();
    let mut queue: Vec<&BinaryTree<char>> = Vec::new();
    queue.push(btree);
    while let Some(node) = queue.pop() {
        match node.get_value() {
            'A'..='Z' | '0' | '1' => result.push(*node.get_value()),
            '!' => {
                result.push(*node.get_value());
                queue.push(node.get_left().expect("! instruction missing operand"));
            }
            '&' | '|' | 'ˆ' | '>' | '=' => {
                result.push(*node.get_value());
                queue.push(node.get_left().expect(&format!(
                    "{} instruction missing left operand",
                    node.get_value()
                )));
                queue.push(node.get_right().expect(&format!(
                    "{} instruction missing left operand",
                    node.get_value()
                )));
            }
            _ => panic!("Invalid character in formula"),
        }
    }
    result.chars().rev().collect()
}
