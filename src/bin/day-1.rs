// Sonar sweep
use std::fs;

const INPUT: &str = "src/inputs/day-1.txt";

fn read_input(input: &str) -> Vec<u32> {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let vectorized: Vec<u32> = str_data
        .split("\n")
        .map(|string| string.parse::<u32>().unwrap())
        .collect();
    vectorized
}

fn sonar_sweep(input: &str) -> usize {
    let data = read_input(input);

    let mut iter = data.iter().peekable();
    let mut relief: Vec<bool> = Vec::new();

    while let Some(val) = iter.next() {
        match iter.peek() {
            Some(&next) => relief.push(next > val),
            None => {}
        }
    }
    relief
        .iter()
        .filter(|&val| *val)
        .map(|_| 1)
        .collect::<Vec<usize>>()
        .len()
}

fn sonar_sweep_sliding_window(input: &str) -> usize {
    let data = read_input(input);

    let mut iter = data.iter().peekable();
    let mut relief: Vec<bool> = Vec::new();

    // Get first sliding window value to compare while iterating
    let first = iter.next().unwrap();
    let mut previous = iter.next().unwrap();
    let mut previous_sum = first + previous;
    match iter.peek() {
        Some(&next) => previous_sum += next,
        None => {}
    }
    // Iterate through values and keep track of current sum for sliding window for next iteration
    while let Some(val) = iter.next() {
        match iter.peek() {
            Some(&next) => {
                let current_sum = previous + val + next;
                relief.push(current_sum > previous_sum);
                previous = val;
                previous_sum = current_sum;
            }
            None => {}
        }
    }
    relief
        .iter()
        .filter(|&val| *val)
        .map(|_| 1)
        .collect::<Vec<usize>>()
        .len()
}

fn main() {
    println!("Number of increases in relief: {}", sonar_sweep(INPUT));
    println!(
        "Number of increases in relief with sliding window: {}",
        sonar_sweep_sliding_window(INPUT)
    );
}
