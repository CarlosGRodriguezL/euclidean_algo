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
