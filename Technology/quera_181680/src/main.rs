use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();

    stdin.lock().read_line(&mut input).unwrap();

    let mut numbers = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<u32>().unwrap());

    let working = numbers.next().unwrap();
    let studding = numbers.next().unwrap();
    let both = numbers.next().unwrap();

    // Do something with the three numbers
    println!("{}", 24 - ((working + studding) - both));
}
