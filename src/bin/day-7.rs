use std::collections::HashMap;
use std::fs;
const INPUT: &str = "src/inputs/day-7.txt";

fn read_input(input: &str) -> Vec<i32> {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let vectorized = str_data
        .split(",")
        .map(|string| string.parse::<i32>().unwrap())
        .collect();
    vectorized
}

fn calculate_fuel_count(x: i32, crab_cache: &HashMap<i32, i32>, distanced: bool) -> i32 {
    match distanced {
        true=> {
            crab_cache.iter().fold(0, |accum, (k,v)| {
                let fuel_sum = (0..((k - x).abs() + 1)).fold(0, |accum, v| v+accum);
                let value = accum + (v * fuel_sum);
                value
            })
        },
        false => {
            crab_cache.iter().fold(0, |accum, (k,v)| {
                let value = accum + (v * (k - x).abs());
                value
            })
        }
    }
}

fn move_crabs_brute_force(input: &str, distanced: bool) -> i32 {
    let mut crabs: Vec<i32> = read_input(input);
    crabs.sort();
    let max = *crabs.last().unwrap() as usize;
    let mut crabs_cache: HashMap<i32, i32> = HashMap::new();
    crabs.iter().for_each(|c| {
        crabs_cache.insert(
            *c,
            if crabs_cache.contains_key(c) {
                crabs_cache[c] + 1
            } else {
                1
            },
        );
    });
    let distance_counters: Vec<i32> = (0..max + 1).map(|i| calculate_fuel_count(i as i32, &crabs_cache, distanced)).collect();
    *distance_counters.iter().min().unwrap()
}

fn main() {
    // Median would be enough for p1, but not for p2 anymore
    println!("Fuel cost {}", move_crabs_brute_force(INPUT, false));
    println!("Crab-engineering fuel cost {}", move_crabs_brute_force(INPUT, true));
}
