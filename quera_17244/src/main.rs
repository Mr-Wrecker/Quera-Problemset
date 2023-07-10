use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    let charge = input.next().unwrap().unwrap().parse::<usize>().unwrap();
    let mut time = 0;

    for i in 0..charge {
        time += i + 1;
    }

    println!("{}", time);
}
