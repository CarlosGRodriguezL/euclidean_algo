use num::*;

pub fn run(x: u64, y: u64) -> u64 {
    let rest = x % y;
    
    if rest > 0 {
        return run(y, rest);
    }
    return y;
}

pub fn run_bigint(x: &BigUint, y: &BigUint) -> BigUint {
    let rest = x.div_rem(&y);

    if !rest.1.is_zero() {
        return run_bigint(&y, &rest.1);
    }
    return y.clone();
}
