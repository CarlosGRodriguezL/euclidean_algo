use num::*;

pub fn run(mut x: u64, mut y: u64) -> u64 {
    loop {
        let r = x % y;
        x = y;
        y = r;
        if y == 0 {
            break x;
        }
    }
}

pub fn run_bigint(mut x: BigUint, mut y: BigUint) -> BigUint {
    loop {
        let r = x.div_rem(&y);
        x = y;
        y = r.1;
        if y.is_zero() {
            break x;
        }
    }
}
