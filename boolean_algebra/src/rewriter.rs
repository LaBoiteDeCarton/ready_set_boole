use super::btree::BinaryTree;

pub fn elimination_of_double_negation(btree: &BinaryTree<char>) -> BinaryTree<char> {
    let instruction = btree.get_value();
    if *instruction == '!' && *btree.get_left().unwrap().get_value() == '!' {
        return elimination_of_double_negation(btree.get_left().unwrap().get_left().unwrap());
    } else {
        let mut new_btree = BinaryTree::new(*instruction);
        if let Some(left) = btree.get_left() {
            new_btree.set_left(elimination_of_double_negation(left));
        }
        if let Some(right) = btree.get_right() {
            new_btree.set_right(elimination_of_double_negation(right));
        }
        return new_btree;
    }
}

pub fn de_morgan_laws(btree: &BinaryTree<char>) -> BinaryTree<char> {
    let instruction = btree.get_value();
    if *instruction == '!' {
        let left_instruction = btree.get_left().unwrap().get_value();
        match left_instruction {
            '&' => {
                let mut new_btree = BinaryTree::new('|');
                let mut left: BinaryTree<char> = BinaryTree::new('!');
                let mut right: BinaryTree<char> = BinaryTree::new('!');
                left.set_left(de_morgan_laws(
                    btree.get_left().unwrap().get_left().unwrap(),
                ));
                right.set_left(de_morgan_laws(
                    btree.get_left().unwrap().get_right().unwrap(),
                ));
                new_btree.set_left(left);
                new_btree.set_right(right);
                return new_btree;
            }
            '|' => {
                let mut new_btree = BinaryTree::new('&');
                let mut left: BinaryTree<char> = BinaryTree::new('!');
                let mut right: BinaryTree<char> = BinaryTree::new('!');
                left.set_left(de_morgan_laws(
                    btree.get_left().unwrap().get_left().unwrap(),
                ));
                right.set_left(de_morgan_laws(
                    btree.get_left().unwrap().get_right().unwrap(),
                ));
                new_btree.set_left(left);
                new_btree.set_right(right);
                return new_btree;
            }
            _ => {
                let mut new_btree = BinaryTree::new(*instruction);
                new_btree.set_left(de_morgan_laws(btree.get_left().unwrap()));
                return new_btree;
            }
        }
    } else {
        let mut new_btree = BinaryTree::new(*instruction);
        if let Some(left) = btree.get_left() {
            new_btree.set_left(de_morgan_laws(left));
        }
        if let Some(right) = btree.get_right() {
            new_btree.set_right(de_morgan_laws(right));
        }
        return new_btree;
    }
}
