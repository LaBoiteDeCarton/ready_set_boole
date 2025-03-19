
pub fn z_coordinate(x: u16, y: u16) -> u32 {
    let mut z: u32 = 0;
    let x: u32 = x.into();
    let y: u32 = y.into();
    for i in 0..16 {
        z = z | (x >> i & 1) << (2 * i) | (y >> i & 1) << (2 * i + 1);
    }
    z
}

pub fn map(x: u16, y: u16) -> f64 {
    let z: f64 = z_coordinate(x, y).into();
    let u32_max: f64 = u32::MAX.into();
    return z / u32_max;
}