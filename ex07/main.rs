use boolean_algebra::formula::*;

fn main() {
    for _ in 0..100 {
        let formula = generate_symbolic_rpn(10);
        println!(
            "{} is {}",
            formula,
            if sat(&formula) {
                "satisfiable 💚"
            } else {
                "unsatisfiable 💔"
            }
        );
    }
    let formula = "AA!&";
    println!(
        "{} is {}",
        formula,
        if sat(&formula) {
            "satisfiable 💚"
        } else {
            "unsatisfiable 💔"
        }
    );
    for _ in 0..1000 {
        let formula = generate_symbolic_rpn(10);
        if !sat(&formula) {
            println!("{} is {}", formula, "unsatisfiable 💔");
        }
    }
}
