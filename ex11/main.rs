mod functions;
// use rand::prelude::*;
use zorder::{index_of};

/**
 * Print all coordinates and their corresponding values
 * Be aware this can take a long time to run and take a lot of memory
 * Actually if you use it, please CTRL+C to stop it after a few seconds
 */
#[allow(dead_code)]
fn print_all_coords() {
    for x in u16::MIN..=u16::MAX {
        for y in u16::MIN..=u16::MAX {
            println!("({},{})->{}", x, y, functions::map(x, y));
        }
    }
}



fn main() {
    let test_cases = vec![
        (0, 0),
        (1, 1),
        (2, 3),
        (5, 10),
        (255, 255),
        (1023, 1023),
        (u16::MAX, u16::MAX),
    ];

    for (x, y) in test_cases {
        let my_result = functions::map(x, y);
        let (reverse_x, reverse_y) = functions::reverse_map(my_result);
        if reverse_x != x || reverse_y != y {
            println!("💔 ({},{}) -> {} -> ({},{})", x, y, my_result, x, y);
        } else {
            println!("💚 ({},{}) -> {} -> ({},{})", x, y, my_result, x, y);
        }
    }
    for _ in 0..100 {
        let x = rand::random::<u16>();
        let y = rand::random::<u16>();
        let my_result = functions::map(x, y);
        let (reverse_x, reverse_y) = functions::reverse_map(my_result);
        if reverse_x != x || reverse_y != y {
            println!("💔 ({},{}) -> {} -> ({},{})", x, y, my_result, x, y);
        } else {
            println!("💚 ({},{}) -> {} -> ({},{})", x, y, my_result, x, y);
        }
    }
}
