pub fn run(x: u64, y: u64) -> u64 {
    let rest = x % y;
    
    if rest > 0 {
        return run(y, rest)
    }
    return y;
}
