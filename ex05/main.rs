mod functions;

fn main() {
    // functions::print_binarytree_char(&functions::create_bool_algebra_binary_tree("AB&"));
    functions::print_binarytree_char(&functions::create_bool_algebra_binary_tree("AAC|B|="));
    // functions::create_reverse_polish_notation_from_btree(
    //     &functions::create_bool_algebra_binary_tree("AAC|B|="),
    // );
}
