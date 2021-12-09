use itertools::Itertools;
use std::collections::HashMap;
use std::fs;

const INPUT: &str = "src/inputs/day-8.txt";

fn read_input(input: &str) -> (Vec<Vec<String>>, Vec<Vec<String>>) {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let mut outputs: Vec<Vec<String>> = Vec::new();
    let vectorized: Vec<Vec<String>> = str_data
        .split("\n")
        .map(|s| {
            let mut split = s.split(" | ");
            let lhs = split.next().unwrap().to_owned();
            let rhs = split.next().unwrap().to_owned();
            let mut lhs_vec: Vec<String> = lhs
                .split(" ")
                .map(|s| s.chars().sorted().collect::<String>())
                .collect();
            let mut rhs_vec: Vec<String> = rhs
                .split(" ")
                .map(|s| s.chars().sorted().collect::<String>())
                .collect();
            outputs.push(rhs_vec.clone());
            lhs_vec.append(&mut rhs_vec);
            lhs_vec
        })
        .collect();
    (vectorized, outputs)
}

fn digit_contains(digit: &str, other: &str) -> bool {
    for s in other.chars() {
        if !digit.contains(s) {
            return false;
        }
    }
    return true;
}

fn subtract_string(digit: String, other: String) -> String {
    digit.chars().filter(|c| !other.contains(*c)).collect()
}

fn reason_dictionary(input: &Vec<String>) -> HashMap<String, i32> {
    let mut digit_value_dict: HashMap<String, i32> = HashMap::new();
    let mut value_digit_dict: HashMap<i32, String> = HashMap::new();
    // 1,4,7,8 can be solved immediately
    input.iter().for_each(|s| {
        match s.len() {
            2 => {
                digit_value_dict.insert(s.clone(), 1);
                value_digit_dict.insert(1, s.clone());
            }
            3 => {
                digit_value_dict.insert(s.clone(), 7);
                value_digit_dict.insert(7, s.clone());
            }
            4 => {
                digit_value_dict.insert(s.clone(), 4);
                value_digit_dict.insert(4, s.clone());
            }
            7 => {
                digit_value_dict.insert(s.clone(), 8);
                value_digit_dict.insert(8, s.clone());
            }
            _ => {}
        };
    });
    // 3 is the string where (x - strings(1)).len()  == 1 and x set in
    let three_string = input
        .iter()
        .find(|s| s.len() == 5 && digit_contains(s, value_digit_dict.get(&1).unwrap()))
        .unwrap();
    digit_value_dict.insert(three_string.into(), 3);
    value_digit_dict.insert(3, three_string.into());
    // 9 is where 8 - 4 - 3 and 4 in x_6 and 3 in x_6
    let nine_string = input
        .iter()
        .find(|s| {
            s.len() == 6
                && digit_contains(s, value_digit_dict.get(&3).unwrap())
                && digit_contains(s, value_digit_dict.get(&4).unwrap())
        })
        .unwrap();
    digit_value_dict.insert(nine_string.into(), 9);
    value_digit_dict.insert(9, nine_string.into());
    let e_true = subtract_string(
        subtract_string(
            value_digit_dict.get(&8).unwrap().into(),
            value_digit_dict.get(&4).unwrap().into(),
        ),
        value_digit_dict.get(&3).unwrap().into(),
    );
    // 2 is where |x| = 5 and x contains e_true and x is not 3
    let two_string = input
        .iter()
        .find(|s| {
            s.len() == 5 && *s != value_digit_dict.get(&3).unwrap() && digit_contains(s, &e_true)
        })
        .unwrap();
    digit_value_dict.insert(two_string.into(), 2);
    value_digit_dict.insert(2, two_string.into());
    // 5 is last where |x| = 5
    let five_string = input
        .iter()
        .find(|s| {
            s.len() == 5
                && *s != value_digit_dict.get(&2).unwrap()
                && *s != value_digit_dict.get(&3).unwrap()
        })
        .unwrap();
    digit_value_dict.insert(five_string.into(), 5);
    // 0 is where 7 is in x_6
    let zero_string = input
        .iter()
        .find(|s| {
            s.len() == 6
                && *s != value_digit_dict.get(&9).unwrap()
                && digit_contains(s, value_digit_dict.get(&7).unwrap())
        })
        .unwrap();
    digit_value_dict.insert(zero_string.into(), 0);
    // 6 remaining
    let six_string = input
        .iter()
        .find(|s| !digit_value_dict.contains_key(*s))
        .unwrap();
    digit_value_dict.insert(six_string.into(), 6);
    digit_value_dict
}

fn output_to_number(output: &Vec<String>, dict: &HashMap<String, i32>) -> i32{
    let translated: Vec<String> = output.iter().map(|s| (*dict.get(s).unwrap()).to_string()).collect();
    let stringified: String = translated.iter().join("");
    stringified.parse::<i32>().unwrap()
}


fn compute_output_sum(input: &str) -> i32 {
    let (all, outputs) = read_input(input);
    let dicts:Vec<HashMap<String, i32>> = all.iter().map(|x| reason_dictionary(x)).collect();
    let mut result_sum = 0;
    for (idx, dict) in dicts.iter().enumerate() {
        let output = outputs[idx].clone();
        result_sum += output_to_number(&output, dict);
    }
    result_sum
}

fn main() {
    // Median would be enough for p1, but not for p2 anymore
    read_input(INPUT);
    println!("Output values: {}", compute_output_sum(INPUT));
}
