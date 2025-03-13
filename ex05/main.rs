use boolean_algebra::btree::*;
use boolean_algebra::*;

fn main() {
    let formula = formula::generate_symbolic_rpn(200);
    println!("{}", formula);
    let binary_tree = conversion::rpn_to_btree(&formula);
    println!("{}", conversion::btree_to_rpn(&binary_tree));
    print_binarytree_char(&binary_tree);
}
