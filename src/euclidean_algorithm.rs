fn euclidean_algorithm(x: u64, y: u64) -> u64 {
    let rest = x % y;
    
    if rest > 0 {
        return euklidische_algo(y, rest);
    }
    return y;
}
