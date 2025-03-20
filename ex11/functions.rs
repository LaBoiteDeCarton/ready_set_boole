
pub fn z_coordinate(x: u16, y: u16) -> u32 {
    let mut z: u32 = 0;
    let x: u32 = x.into();
    let y: u32 = y.into();
    for i in 0..16 {
        z = z | (x >> i & 1) << (2 * i) | (y >> i & 1) << (2 * i + 1);
    }
    z
}

pub fn reverse_z_coordinate(z: u32) -> (u16, u16) {
    let mut x: u16 = 0;
    let mut y: u16 = 0;
    for i in 0..16 {
        x = x | ((z >> (2 * i) & 1) << i) as u16;
        y = y | ((z >> (2 * i + 1) & 1) << i) as u16;
    }
    (x, y)
}

pub fn map(x: u16, y: u16) -> f64 {
    let z: f64 = z_coordinate(x, y).into();
    let u32_max: f64 = u32::MAX.into();
    return z / u32_max;
}

pub fn reverse_map(n: f64) -> (u16, u16) {
    let z = (n * u32::MAX as f64) as u32;
    return reverse_z_coordinate(z);
}