use itertools::Itertools;
use std::error::Error;
use std::fmt;
use std::fs;

const INPUT: &str = "src/inputs/day-3.txt";

fn read_input(input: &str) -> String {
    fs::read_to_string(input).expect("Unable to open input")
}

struct Diagnostic<const BIT: usize> {
    accum: Vec<i32>,
}

impl<const BIT: usize> Default for Diagnostic<BIT> {
    fn default() -> Diagnostic<BIT> {
        Diagnostic {
            accum: vec![0; BIT],
        }
    }
}

impl<const BIT: usize> Diagnostic<BIT> {
    fn new(values: Vec<i32>) -> Diagnostic<BIT> {
        Diagnostic { accum: values }
    }

    fn add(self, values: Vec<i32>) -> Diagnostic<BIT> {
        let mut accum = Vec::new();
        values.iter().enumerate().for_each(|(i, x)| {
            let mut val = 1;
            if *x == 0 {
                val = -1;
            }
            accum.push(self.accum[i] + val);
        });
        Diagnostic { accum }
    }

    fn values_from_str(string: &str) -> Result<Vec<i32>, AOCError> {
        if string.len() != BIT {
            return Err(AOCError::new("Wrong diagnostic string length"));
        }
        let mut diagnostic_values = Vec::new();
        for i in 0..BIT {
            let parsed_val = string
                .chars()
                .nth(i)
                .unwrap()
                .to_string()
                .parse::<i32>()
                .ok()
                .unwrap();
            diagnostic_values.push(parsed_val);
        }
        Ok(diagnostic_values)
    }

    fn gamma_rate(&self) -> i32 {
        let mut gamma = Vec::new();
        for i in 0..BIT {
            gamma.push((self.accum[i] > 0) as i32);
        }
        Diagnostic::<BIT>::vec_to_integer(gamma)
    }

    fn epsilon_rate(&self) -> i32 {
        let mut epsilon = Vec::new();
        for i in 0..BIT {
            epsilon.push((self.accum[i] < 0) as i32);
        }
        Diagnostic::<BIT>::vec_to_integer(epsilon)
    }

    fn vec_to_integer(vec: Vec<i32>) -> i32 {
        let str_representation = vec.iter().join("");
        isize::from_str_radix(&str_representation, 2).unwrap() as i32
    }
}

fn calculate_power_consumption(input_path: &str) -> i32 {
    let str_data = read_input(input_path);

    let accum_diagnostic: Diagnostic<12> = str_data
        .split('\n')
        .map(|str_data| Diagnostic::<12>::values_from_str(str_data).ok().unwrap())
        .fold(Diagnostic::<12>::default(), |acc, diag| acc.add(diag));

    let gamma = accum_diagnostic.gamma_rate();
    let epsilon = accum_diagnostic.epsilon_rate();

    gamma * epsilon
}

fn calculate_life_support(input_path: &str) -> i32 {
    let str_data = read_input(input_path);
    // Parse input
    let diagnostics: Vec<Diagnostic<12>> = str_data
        .split('\n')
        .map(|str_data| {
            Diagnostic::<12>::new(Diagnostic::<12>::values_from_str(str_data).ok().unwrap())
        })
        .collect();

    0
}

fn main() {
    println!("Power consumption: {}", calculate_power_consumption(INPUT));
    println!("Life support value: {}", calculate_life_support(INPUT));
}

#[derive(Debug)]
struct AOCError {
    details: String,
}

impl AOCError {
    fn new(msg: &str) -> AOCError {
        AOCError {
            details: msg.to_string(),
        }
    }
}

impl fmt::Display for AOCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.details)
    }
}

impl Error for AOCError {
    fn description(&self) -> &str {
        &self.details
    }
}
