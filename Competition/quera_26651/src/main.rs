use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Number of Question
    let number = input.next().unwrap().unwrap().parse::<usize>().unwrap();

    // Question Breathing rate
    let line2 = input.next().unwrap().unwrap();
    let questions = line2
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    // Person Solve The Question
    let line3 = input.next().unwrap().unwrap();
    let persons = line3
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap());

    // Total breaths taken
    let sum: usize = questions
        .zip(persons)
        .take(number)
        .map(|(q, p)| q * p)
        .sum();

    println!("{}", sum);
}
