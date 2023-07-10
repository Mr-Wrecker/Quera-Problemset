use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let line1 = input.next().unwrap().unwrap();
    let mut line1_part = line1.trim().split_whitespace();

    // Number of jars of jam
    let n = line1_part.next().unwrap().parse::<f32>().unwrap();

    // Jam jar capacity
    let k = line1_part.next().unwrap().parse::<f32>().unwrap();

    // The volume of available jam
    let sum: f32 = input
        .next()
        .unwrap()
        .unwrap()
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<f32>().unwrap())
        .sum();

    println!("{}", (n - (sum / k)) as usize);
}
