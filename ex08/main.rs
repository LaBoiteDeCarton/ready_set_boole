use boolean_algebra::set::*;

fn main() {
    println!("{:?}", powerset(vec![1, 2, 3]));
    println!("{:?}", powerset(vec![1, 2, 3, 4]));
    println!("{:?}", powerset(vec![42]));
}
