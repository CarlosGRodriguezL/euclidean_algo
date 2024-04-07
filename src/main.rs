use std::io;
use std::time::Instant;

fn main() {
    let mut x: u64 = request_param("Gib die erste Zahl ein: ");
    let mut y: u64 = request_param("Gib die zweite Zahl ein: ");
    if x < y {
        (x, y) = (y, x);
    }
    println!("Starte euklidischen Algorithmus:");
    let start_time = Instant::now();
    let ggt: u64 = euklidische_algo(x, y);
    let end_time = Instant::now();
    let elapsed_time = end_time.duration_since(start_time).as_secs_f64();
    println!("Der größte gemeinsame Teiler von {x} und {y} ist {ggt}!");
    println!("Der euklidische Algorithmus hat {elapsed_time} Sekunden gedauert.");
    let kgv: u128 = kg_vielfaches(x.try_into().unwrap(), y.try_into().unwrap(), ggt.try_into().unwrap());
    println!("Das kleinste gemeinsame Vielfache von {x} und {y} ist {kgv}!")
}

fn request_param(text: &str) -> u64 {
    let mut input_line = String::new();
    println!("{text}");
    io::stdin()
        .read_line(&mut input_line)
        .expect("Input konnte nicht gelesen werden.");
    return input_line
        .trim()
        .parse()
        .expect("Input ist keine Zahl.");
}

fn euklidische_algo(x: u64, y: u64) -> u64 {
    let rest = x % y;
    let teiler = (x-rest)/y;
    
    println!("-> {x} dividiert durch {y} ist {teiler} mit {rest} Rest.");

    if rest > 0 {
        return euklidische_algo(y, rest);
    }
    return y;
}

fn kg_vielfaches(x: u128, y: u128, ggt: u128) -> u128 {
    return (x/ggt)*y;
}
