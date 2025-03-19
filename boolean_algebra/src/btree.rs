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
        if *btree.get_value() == '!' {
            rec_print_binarytree_char(left, level + 1, &new_prefix, false);
            return;
        }
        rec_print_binarytree_char(left, level + 1, &new_prefix, true);
    }
    if let Some(right) = btree.get_right() {
        rec_print_binarytree_char(right, level + 1, &new_prefix, false);
    }
}

pub fn print_binarytree_char(btree: &BinaryTree<char>) {
    rec_print_binarytree_char(btree, 0, "", false);
}
