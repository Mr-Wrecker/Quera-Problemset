// input -> output

// 3 2 1 3 -> 0 0 0 0
// 2 1 3 2 -> 1 0 0 0   with rotation
// 2 2 1 3              outwith rotation
// 0 3 2 2 -> 1 1 0 0   with rotation
// 2 2 0 3              outwith rotation

// 3 3 5 3 -> 0 0 0 0
// 3 5 3 2 -> 1 0 0 0   with rotation
// 2 3 5 3              outwith rotation
// 4 3 2 3 -> 1 1 0 0   with rotation
// 2 3 4 3              outwith rotation
// 3 1 3 4 -> 1 1 1 0   with rotation
// 1 3 4 3              outwith rotation
// 1 3 3 3 -> 1 1 1 1   with rotation
// 1 3 3 3              outwith rotation
// 3 3 3 0 -> 2 1 1 1   with rotation
// 0 3 3 3              outwith rotation

// By This Trace, Just Value of index 0 and 2 in each round decreased

use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();

    // Parse Input
    let input_values: Vec<usize> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    let mut scope1 = input_values[0];
    let mut scope2 = input_values[2];
    let mut output = vec![0, 0, 0, 0];

    let mut flag = true;
    let mut counter = 0;

    // Count number of chocolates eaten
    while scope1 != 0 && scope2 != 0 {
        if flag {
            scope1 -= 1;
        } else {
            scope2 -= 1;
        }

        flag = !flag;
        output[counter] += 1;
        counter = (counter + 1) % 4
    }

    println!("{} {} {} {}", output[0], output[1], output[2], output[3]);
}
