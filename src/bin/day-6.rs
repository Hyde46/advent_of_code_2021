use std::fs;

const INPUT: &str = "src/inputs/day-6.txt";

fn read_input(input: &str) -> Vec<u64> {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let vectorized = str_data
        .split(",")
        .map(|string| string.parse::<u64>().unwrap())
        .collect();
    vectorized
}

fn simulate_cached_fish(input: &str, days: usize) -> usize {
    let fish: Vec<u64> = read_input(input);
    // 9 possible fish states
    let mut fish_cache: Vec<usize> = vec![0; 9];
    let mut fish_swap: Vec<usize> = vec![0; 9];
    fish.iter()
        .for_each(|f| fish_cache[*f as usize] = fish_cache[*f as usize] + 1);
    for _i in 1..days + 1 {
        fish_cache.iter().enumerate().for_each(|(i, f)| {
            let target_swap_index = if i == 0 { 6 } else { i - 1 };
            fish_swap[target_swap_index] = if fish_swap[target_swap_index] == 0 {
                *f
            } else {
                *f + fish_swap[target_swap_index]
            };
            if i == 0 {
                fish_swap[8] = *f;
            }
        });
        fish_cache = fish_swap.clone();
        // clear swap
        fish_swap = vec![0; 9];
    }
    fish_cache.iter().fold(0, |acc, f| acc + *f)
}

fn main() {
    println!(
        "Cached fish simulation {}",
        simulate_cached_fish(INPUT, 256)
    );
}
