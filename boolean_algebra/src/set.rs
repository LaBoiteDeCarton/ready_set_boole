use crate::formula;
use rand::seq::SliceRandom;

pub fn powerset(set: Vec<i32>) -> Vec<Vec<i32>> {
    let mut result: Vec<Vec<i32>> = Vec::new();
    result.push(Vec::new());
    for element in set {
        let mut subsets = result.clone();
        for subset in subsets.iter_mut() {
            subset.push(element);
        }
        result.append(&mut subsets);
    }
    return result;
}

fn set_union<T: Clone + std::cmp::PartialEq>(set1: Vec<T>, set2: Vec<T>) -> Vec<T> {
    let mut result = set1.clone();
    for element in set2 {
        if !result.contains(&element) {
            result.push(element);
        }
    }
    return result;
}

fn set_intersection<T>(set1: Vec<T>, set2: Vec<T>) -> Vec<T>
where
    T: Clone + std::cmp::PartialEq,
{
    let mut result: Vec<T> = Vec::new();
    for element in set1 {
        if set2.contains(&element) {
            result.push(element);
        }
    }
    return result;
}

fn set_complement<T>(set1: Vec<T>, sets: &Vec<Vec<T>>) -> Vec<T>
where
    T: Clone + std::cmp::PartialEq,
{
    let mut result: Vec<T> = Vec::new();
    for set in sets {
        for element in set {
            if !set1.contains(element) && !result.contains(element) {
                result.push(element.clone());
            }
        }
    }
    return result;
}

pub fn eval_set(formula: &str, sets: &Vec<Vec<i32>>) -> Vec<i32> {
    let formula_nnf = formula::negation_normal_form(formula);
    let mut setsqueue = Vec::new();

    for next_instruction in formula_nnf.chars() {
        match next_instruction {
            'A'..='Z' => {
                //find index of next_instruction in ['A'..'Z']
                let index = next_instruction as usize - 'A' as usize;
                let set = sets.get(index).expect("Set not found").clone();
                setsqueue.push(set);
            }
            '!' => {
                let a = setsqueue.pop().expect("! instruction missing operand");
                setsqueue.push(set_complement(a, sets));
            }
            '&' => {
                let a = setsqueue.pop().expect("& instruction missing left operand");
                let b = setsqueue
                    .pop()
                    .expect("& instruction missing right operand");
                setsqueue.push(set_intersection(a, b));
            }
            '|' => {
                let a = setsqueue.pop().expect("| instruction missing left operand");
                let b = setsqueue
                    .pop()
                    .expect("| instruction missing right operand");
                setsqueue.push(set_union(a, b));
            }
            _ => panic!("Invalid character in formula"),
        }
    }

    let result = setsqueue
        .pop()
        .expect("Formula did not result in a single boolean value");
    if setsqueue.is_empty() {
        result
    } else {
        panic!("Formula did not result in a single boolean value");
    }
}

pub fn generate_sets(size: usize, max_element: i32) -> Vec<Vec<i32>> {
    let mut rng = rand::rng();
    let powerset = powerset((1..=max_element).collect::<Vec<_>>());
    let mut sets_to_shuffle: Vec<Vec<i32>> = powerset.clone().into_iter().collect();
    sets_to_shuffle.shuffle(&mut rng);
    sets_to_shuffle.truncate(size);
    return sets_to_shuffle;
}