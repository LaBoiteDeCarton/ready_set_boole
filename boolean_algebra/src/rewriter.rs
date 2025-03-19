use super::btree::BinaryTree;

pub fn elimination_of_double_negation(btree: &BinaryTree<char>) -> (BinaryTree<char>, bool) {
    let instruction = btree.get_value();
    if *instruction == '!' && *btree.get_left().unwrap().get_value() == '!' {
        let (new_btree, _) =
            elimination_of_double_negation(btree.get_left().unwrap().get_left().unwrap());
        return (new_btree, true);
    } else {
        let mut new_btree = BinaryTree::new(*instruction);
        let mut result_bool = false;
        if let Some(left) = btree.get_left() {
            let (new_left, bool_left) = elimination_of_double_negation(left);
            result_bool = result_bool || bool_left;
            new_btree.set_left(new_left);
        }
        if let Some(right) = btree.get_right() {
            let (new_right, bool_right) = elimination_of_double_negation(right);
            result_bool = result_bool || bool_right;
            new_btree.set_right(new_right);
        }
        return (new_btree, result_bool);
    }
}

pub fn de_morgan_laws(btree: &BinaryTree<char>) -> (BinaryTree<char>, bool) {
    let instruction = btree.get_value();
    if *instruction == '!' {
        let left_instruction = btree.get_left().unwrap().get_value();
        match left_instruction {
            '&' => {
                let mut new_btree = BinaryTree::new('|');
                let mut left: BinaryTree<char> = BinaryTree::new('!');
                let mut right: BinaryTree<char> = BinaryTree::new('!');
                left.set_left(de_morgan_laws(btree.get_left().unwrap().get_left().unwrap()).0);
                right.set_left(de_morgan_laws(btree.get_left().unwrap().get_right().unwrap()).0);
                new_btree.set_left(left);
                new_btree.set_right(right);
                return (new_btree, true);
            }
            '|' => {
                let mut new_btree = BinaryTree::new('&');
                let mut left: BinaryTree<char> = BinaryTree::new('!');
                let mut right: BinaryTree<char> = BinaryTree::new('!');
                left.set_left(de_morgan_laws(btree.get_left().unwrap().get_left().unwrap()).0);
                right.set_left(de_morgan_laws(btree.get_left().unwrap().get_right().unwrap()).0);
                new_btree.set_left(left);
                new_btree.set_right(right);
                return (new_btree, true);
            }
            _ => {
                let mut new_btree = BinaryTree::new(*instruction);
                let (left, bool_left) = de_morgan_laws(btree.get_left().unwrap());
                new_btree.set_left(left);
                return (new_btree, bool_left);
            }
        }
    } else {
        let mut new_btree = BinaryTree::new(*instruction);
        let mut result_bool = false;
        if let Some(left) = btree.get_left() {
            let (new_left, bool_left) = de_morgan_laws(left);
            new_btree.set_left(new_left);
            result_bool = result_bool || bool_left;
        }
        if let Some(right) = btree.get_right() {
            let (new_right, bool_right) = de_morgan_laws(right);
            new_btree.set_right(new_right);
            result_bool = result_bool || bool_right;
        }
        return (new_btree, result_bool);
    }
}

pub fn elimination_material_conditions(btree: &BinaryTree<char>) -> BinaryTree<char> {
    let instruction = btree.get_value();
    if *instruction == '>' {
        let mut new_btree = BinaryTree::new('|');
        let mut left: BinaryTree<char> = BinaryTree::new('!');
        left.set_left(elimination_material_conditions(btree.get_left().unwrap()));
        new_btree.set_left(left);
        new_btree.set_right(elimination_material_conditions(btree.get_right().unwrap()));
        return new_btree;
    } else {
        let mut new_btree = BinaryTree::new(*instruction);
        if let Some(left) = btree.get_left() {
            new_btree.set_left(elimination_material_conditions(left));
        }
        if let Some(right) = btree.get_right() {
            new_btree.set_right(elimination_material_conditions(right));
        }
        return new_btree;
    }
}

pub fn elimination_equvalence(btree: &BinaryTree<char>) -> BinaryTree<char> {
    let instruction = btree.get_value();
    if *instruction == '=' {
        let mut new_btree = BinaryTree::new('&');
        let mut left: BinaryTree<char> = BinaryTree::new('>');
        let mut right: BinaryTree<char> = BinaryTree::new('>');
        left.set_left(elimination_equvalence(btree.get_left().unwrap()));
        left.set_right(elimination_equvalence(btree.get_right().unwrap()));
        right.set_left(elimination_equvalence(btree.get_right().unwrap()));
        right.set_right(elimination_equvalence(btree.get_left().unwrap()));
        new_btree.set_left(left);
        new_btree.set_right(right);
        return new_btree;
    } else {
        let mut new_btree = BinaryTree::new(*instruction);
        if let Some(left) = btree.get_left() {
            new_btree.set_left(elimination_equvalence(left));
        }
        if let Some(right) = btree.get_right() {
            new_btree.set_right(elimination_equvalence(right));
        }
        return new_btree;
    }
}

pub fn disjonctive_distributivity(btree: &BinaryTree<char>) -> (BinaryTree<char>, bool) {
    let instruction = btree.get_value();
    if *instruction == '|' {
        let left_instruction = btree.get_left().unwrap().get_value();
        let right_instruction = btree.get_right().unwrap().get_value();
        if *left_instruction == '&' {
            let mut new_btree = BinaryTree::new('&');
            let mut left: BinaryTree<char> = BinaryTree::new('|');
            let mut right: BinaryTree<char> = BinaryTree::new('|');
            left.set_left(
                disjonctive_distributivity(btree.get_left().unwrap().get_left().unwrap()).0,
            );
            left.set_right(disjonctive_distributivity(btree.get_right().unwrap()).0);
            right.set_left(
                disjonctive_distributivity(btree.get_left().unwrap().get_right().unwrap()).0,
            );
            right.set_right(disjonctive_distributivity(btree.get_right().unwrap()).0);
            new_btree.set_left(left);
            new_btree.set_right(right);
            return (new_btree, true);
        } else if *right_instruction == '&' {
            let mut new_btree = BinaryTree::new('&');
            let mut left: BinaryTree<char> = BinaryTree::new('|');
            let mut right: BinaryTree<char> = BinaryTree::new('|');
            left.set_left(disjonctive_distributivity(btree.get_left().unwrap()).0);
            left.set_right(
                disjonctive_distributivity(btree.get_right().unwrap().get_left().unwrap()).0,
            );
            right.set_left(disjonctive_distributivity(btree.get_left().unwrap()).0);
            right.set_right(
                disjonctive_distributivity(btree.get_right().unwrap().get_right().unwrap()).0,
            );
            new_btree.set_left(left);
            new_btree.set_right(right);
            return (new_btree, true);
        } else {
            let mut new_btree = BinaryTree::new(*instruction);
            let mut bool_result = false;
            if let Some(left) = btree.get_left() {
                let (new_left, bool_left) = disjonctive_distributivity(left);
                new_btree.set_left(new_left);
                bool_result = bool_result || bool_left;
            }
            if let Some(right) = btree.get_right() {
                let (new_right, bool_right) = disjonctive_distributivity(right);
                new_btree.set_right(new_right);
                bool_result = bool_result || bool_right;
            }
            return (new_btree, bool_result);
        }
    } else {
        let mut new_btree = BinaryTree::new(*instruction);
        let mut bool_result = false;
        if let Some(left) = btree.get_left() {
            let (new_left, bool_left) = disjonctive_distributivity(left);
            new_btree.set_left(new_left);
            bool_result = bool_result || bool_left;
        }
        if let Some(right) = btree.get_right() {
            let (new_right, bool_right) = disjonctive_distributivity(right);
            new_btree.set_right(new_right);
            bool_result = bool_result || bool_right;
        }
        return (new_btree, bool_result);
    }
}

pub fn elimination_xor(btree: &BinaryTree<char>) -> BinaryTree<char> {
    let instruction = btree.get_value();
    if *instruction == 'ˆ' {
        let mut new_btree = BinaryTree::new('&');
        let mut left: BinaryTree<char> = BinaryTree::new('|');
        let mut right: BinaryTree<char> = BinaryTree::new('!');
        let mut right_left: BinaryTree<char> = BinaryTree::new('&');
        left.set_left(elimination_xor(btree.get_left().unwrap()));
        left.set_right(elimination_xor(btree.get_right().unwrap()));
        right_left.set_left(elimination_xor(btree.get_left().unwrap()));
        right_left.set_right(elimination_xor(btree.get_right().unwrap()));
        right.set_left(right_left);
        new_btree.set_left(left);
        new_btree.set_right(right);
        return new_btree;
    } else {
        let mut new_btree = BinaryTree::new(*instruction);
        if let Some(left) = btree.get_left() {
            new_btree.set_left(elimination_xor(left));
        }
        if let Some(right) = btree.get_right() {
            new_btree.set_right(elimination_xor(right));
        }
        return new_btree;
    }
}

pub fn btree_to_fnn(btree: &BinaryTree<char>) -> BinaryTree<char> {
    let mut new_btree = elimination_xor(&elimination_material_conditions(&elimination_equvalence(
        btree,
    )));
    let mut changes_occured = true;
    while changes_occured {
        let changes: bool;
        (new_btree, changes_occured) = elimination_of_double_negation(&new_btree);
        (new_btree, changes) = de_morgan_laws(&new_btree);
        changes_occured = changes_occured || changes;
    }
    return new_btree;
}

pub fn btree_to_cnf(btree: &BinaryTree<char>) -> BinaryTree<char> {
    let mut new_btree = elimination_xor(&elimination_material_conditions(&elimination_equvalence(
        btree,
    )));
    let mut changes_occured = true;
    let mut occurences = 0;
    while changes_occured && occurences < 10 {
        let elimination_of_double_negation_changes: bool;
        let de_morgan_laws_changes: bool;
        let disjonctive_distributivity_changes: bool;
        (new_btree, elimination_of_double_negation_changes) =
            elimination_of_double_negation(&new_btree);
        (new_btree, de_morgan_laws_changes) = de_morgan_laws(&new_btree);
        (new_btree, disjonctive_distributivity_changes) = disjonctive_distributivity(&new_btree);
        changes_occured = elimination_of_double_negation_changes
            || de_morgan_laws_changes
            || disjonctive_distributivity_changes;
        occurences += 1;
    }
    return new_btree;
}
