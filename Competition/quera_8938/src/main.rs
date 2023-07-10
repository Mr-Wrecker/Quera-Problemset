use std::io;

fn main() {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.read_line(&mut input).unwrap();

    // Read Area and Trips Number
    let mut input_iter = input.trim().split_whitespace();
    let areas_number = input_iter.next().unwrap().parse::<usize>().unwrap();
    let trips_number = input_iter.next().unwrap().parse::<usize>().unwrap();

    // Create Matrix of Trip Cost Between Each Area
    let mut cost_matrix: Vec<Vec<i32>> = vec![vec![0; trips_number]; areas_number];
    for i in 0..areas_number {
        let mut row_input = String::new();
        stdin.read_line(&mut row_input).unwrap();
        let row_matrix = row_input
            .split_whitespace()
            .map(|x| x.parse::<i32>().unwrap())
            .collect();
        cost_matrix[i] = row_matrix;
    }

    // Read List of Trips
    let mut trips: Vec<(usize, usize)> = vec![];
    for _ in 0..trips_number {
        let mut source_destination = String::new();
        stdin.read_line(&mut source_destination).unwrap();
        let source_destination_parsed: Vec<usize> = source_destination
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        let source: usize = source_destination_parsed[0];
        let destination = source_destination_parsed[1];
        trips.push((source - 1, destination - 1));
    }

    // Calculate Total Cost
    let cost: i32 = trips
        .iter()
        .map(|&(source, destination)| cost_matrix[source][destination])
        .sum();

    println!("{}", cost);
}
