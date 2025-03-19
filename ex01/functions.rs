pub fn adder(a: u32, b: u32) -> u32 {
    let mut first: u32 = a;
    let mut second: u32 = b;

    while second != 0 {
        (first, second) = (first ^ second, (first & second) << 1);
    }
    first
}

pub fn multiplier(a: u32, b: u32) -> u32 {
    if (a == 0) || (b == 0) {
        return 0;
    }
    let mut result: u32 = 0;
    let (bigger, mut smaller) = if a > b { (a, b) } else { (b, a) };

    for i in 0..32 {
        if smaller == 0 {
            break;
        }
        if smaller & 1 != 0 {
            result = adder(result, bigger << i);
        }
        smaller = smaller >> 1;
    }
    result
}
