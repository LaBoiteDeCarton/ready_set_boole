pub fn adder(a: u32, b: u32) -> u32 {
    let mut first: u32 = a;
    let mut second: u32 = b;

    while second != 0 {
        (first, second) = (first ^ second, (first & second) << 1);
    }
    first
}
