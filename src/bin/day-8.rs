use std::fs;

const INPUT: &str = "src/inputs/day-8.txt";

fn read_input(input: &str) -> Vec<Vec<String>> {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let vectorized: Vec<Vec<String>> = str_data
        .split("\n")
        .map(|s| {
            let mut split = s.split(" | ");
            let output_values = split.nth(1).unwrap().to_owned();
            output_values.split(" ").map(|s| s.to_owned()).collect()
        })
        .collect();
    vectorized
}
fn count_easy_digits(input: &str) -> i32 {
    let output_values = read_input(input);
    output_values.iter().fold(0, |acc, numbers| {
        numbers.iter().fold(0, |num_acc, n| match n.len() {
            2 | 3 | 4 | 7 => num_acc + 1,
            _ => num_acc + 0,
        }) + acc
    })
}

fn main() {
    // Median would be enough for p1, but not for p2 anymore
    println!("Count of 1,4,7,8: {:?}", count_easy_digits(INPUT));
}
