use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::{Add, Mul, Neg};
use std::str::FromStr;

const INPUT: &str = "src/inputs/day-4-example.txt";

struct BingoBoard {
    values: Vec<i32>
}

impl BingoBoard {
    fn new(values: Vec<i32>) -> BingoBoard {
        BingoBoard {
            values
        }
    }
}

fn read_input(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let str_data = fs::read_to_string(input).expect("Unable to open input");

    let mut split_string: Vec<String> = str_data
        .split("\n").map(|s| s.to_owned()).filter(|x| !x.is_empty()).collect();
    let numbers:Vec<i32> = split_string[0].clone().split(',').map(|s| s.parse::<i32>().unwrap()).collect();
    split_string.remove(0);
    
    (numbers, Vec::new())
}

fn find_bingo_winner(input_path: &str) -> i32 {
    let (numbers, bingo_boards): (Vec<i32>, Vec<BingoBoard>) = read_input(input_path);
    0
}

fn main() {
    println!("Diving result: {}", find_bingo_winner(INPUT));
}
