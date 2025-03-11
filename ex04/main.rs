mod functions;

fn main() {
    functions::print_truth_table("AB&");
    println!();
    functions::print_truth_table("AB|");
    println!();
    functions::print_truth_table("AB>");
    println!();
    functions::print_truth_table("AB=");
    println!();
    functions::print_truth_table("ABAC||=");
    println!();
    functions::print_truth_table("AZAA&A&&&");
    println!();
    functions::print_truth_table("H!");
    println!();
}
