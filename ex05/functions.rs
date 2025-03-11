pub struct BinaryTree<T> {
    value: T,
    left: Option<Box<BinaryTree<T>>>,
    right: Option<Box<BinaryTree<T>>>,
}

impl<T> BinaryTree<T> {
    pub fn new(value: T) -> Self {
        BinaryTree {
            value,
            left: None,
            right: None,
        }
    }

    pub fn set_left(&mut self, left: BinaryTree<T>) {
        self.left = Some(Box::new(left));
    }

    pub fn set_right(&mut self, right: BinaryTree<T>) {
        self.right = Some(Box::new(right));
    }

    pub fn get_left(&self) -> Option<&BinaryTree<T>> {
        self.left.as_deref()
    }

    pub fn get_right(&self) -> Option<&BinaryTree<T>> {
        self.right.as_deref()
    }

    pub fn get_value(&self) -> &T {
        &self.value
    }
}

pub fn create_bool_algebra_binary_tree(formula: &str) -> BinaryTree<char> {
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

// pub fn create_reverse_polish_notation_from_btree(btree: &BinaryTree<char>) -> String {
//     let mut result = String::new();
//     let mut queue: Vec<&BinaryTree<char>> = Vec::new();
//     queue.push(btree);
//     while let Some(node) = queue.pop() {
//         match node.get_value() {
//             'A'..='Z' | '0' | '1' => result.insert(0, *node.get_value()),
//             '!' => {
//                 result.insert(0, *node.get_value());
//                 result.insert(0, *node.get_value());
//                 continue;
//                     queue.push(left);
//                 }
//             }
//             '&' | '|' | 'ˆ' | '>' | '=' => {
//                 result.push(*node.get_value());
//                 if let Some(left) = node.get_left() {
//                     queue.push(left);
//                 }
//                 if let Some(right) = node.get_right() {
//                     queue.push(right);
//                 }
//             }
//             _ => panic!("Invalid character in formula"),
//         }
//     }
//     result
// }

fn rec_print_binarytree_char(btree: &BinaryTree<char>, level: usize, prefix: &str, is_left: bool) {
    println!(
        "{}{}{}",
        prefix,
        if level == 0 {
            ""
        } else if is_left {
            "├── "
        } else {
            "└── "
        },
        btree.get_value()
    );

    let new_prefix = format!(
        "{}{}",
        prefix,
        if level == 0 {
            ""
        } else if is_left {
            "│   "
        } else {
            "    "
        }
    );

    if let Some(left) = btree.get_left() {
        rec_print_binarytree_char(left, level + 1, &new_prefix, true);
    }
    if let Some(right) = btree.get_right() {
        rec_print_binarytree_char(right, level + 1, &new_prefix, false);
    }
}

pub fn print_binarytree_char(btree: &BinaryTree<char>) {
    rec_print_binarytree_char(btree, 0, "", false);
}

// pub fn eliminate_double_negation(btree: &BinaryTree<Char>) -> BinaryTree<Char> {
//     let mut result: Vec<String> = Vec::new();sd
//     for next_instruction in formula.chars() {
//         match next_instruction {
//             '!' => {
//                 if result.last().unwrap() == "!" {
//                     result.pop();
//                 } else if result.last().unwrap() == "^" {
//                 } else {
//                     result.push(next_instruction.to_string());
//                 }
//             }
//             _ => result.push(next_instruction.to_string()),
//         }
//     }
// }
