use boolean_algebra::btree::*;
use boolean_algebra::*;

fn one_check_verbose() {
    let formula = formula::generate_symbolic_rpn(10);
    println!("{}", formula);
    let binary_tree = conversion::rpn_to_btree(&formula);
    println!("{}", conversion::btree_to_rpn(&binary_tree));
    print_binarytree_char(&binary_tree);
    let btree_fnn = rewriter::btree_to_fnn(&binary_tree);
    print_binarytree_char(&btree_fnn);
    truth_table::print_truth_table(&formula);
    println!();
    let fnn_formula = conversion::btree_to_rpn(&btree_fnn);
    truth_table::print_truth_table(fnn_formula.as_str());
    println!();
    if formula::formula_equivalence(&formula, fnn_formula.as_str()) {
        println!("💚 The formulas are equivalent");
    } else {
        println!("💔 The formulas are not equivalent");
    }
}

fn one_check() {
    let formula: String = formula::generate_symbolic_rpn(10);
    let fnn_formula = formula::negation_normal_form(formula.as_str());
    if formula::formula_equivalence(&formula, fnn_formula.as_str()) {
        println!("Original formula   : {}", formula);
        println!("FNN formula        : {}", fnn_formula);
        println!("💚 The formulas are equivalent");
        println!()
    } else {
        println!("Original formula: {}", formula);
        println!("FNN formula: {}", fnn_formula);
        println!("💔 The formulas are not equivalent");
        println!()
    }
}

fn multiple_random_check() {
    for _ in 0..100 {
        one_check();
    }
}

fn main() {
    one_check_verbose();
    multiple_random_check();
}
