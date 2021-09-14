pub const N: u8 = 1 << 0;
pub const S: u8 = 1 << 1;
pub const E: u8 = 1 << 2;
pub const W: u8 = 1 << 3;

pub const fn get_dx(dir: u8) -> i64 {
    match dir {
        E => 1,
        W => -1,
        _ => 0,
    }
}

pub const fn get_dy(dir: u8) -> i64 {
    match dir {
        N => -1,
        S => 1,
        _ => 0,
    }
}

pub const fn get_opposite(dir: u8) -> u8 {
    match dir {
        E => W,
        W => E,
        N => S,
        S => N,
        _ => 0,
    }
}
