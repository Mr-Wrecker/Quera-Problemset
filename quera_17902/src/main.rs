use std::io::{self, BufRead};
fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Number of wheels
    let number = input.next().unwrap().unwrap().parse::<usize>().unwrap();

    // The Password
    let password: Vec<char> = input.next().unwrap().unwrap().chars().collect();

    // Number of wheel rotations
    let mut rotation = 0;

    for index in 0..number {
        let wheel = input.next().unwrap().unwrap();
        let position = wheel.find(password[index]).unwrap();
        rotation += if position > 4 { 9 - position } else { position };
    }

    println!("{}", rotation);
}
