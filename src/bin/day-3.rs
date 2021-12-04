use itertools::Itertools;
use std::error::Error;
use std::fmt;
use std::fs;

const INPUT: &str = "src/inputs/day-3.txt";
const BIT_COUNT: usize = 12;
fn read_input(input: &str) -> String {
    fs::read_to_string(input).expect("Unable to open input")
}

#[derive(Debug, Clone)]
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

fn most_common_value_at_pos(vec: &Vec<Diagnostic<BIT_COUNT>>, pos: i32, default_at_0: i32) -> i32 {
    let result = vec.iter().fold(0, |accum, x| {
        let value = x.accum[pos as usize];
        match value {
            0 => accum - 1,
            1 => accum + 1,
            _ => accum,
        }
    });
    if result == 0 {
        return default_at_0;
    }
    (result > 0) as i32
}


fn least_common_value_at_pos(vec: &Vec<Diagnostic<BIT_COUNT>>, pos: i32, default_at_0: i32) -> i32 {
    let result = vec.iter().fold(0, |accum, x| {
        let value = x.accum[pos as usize];
        match value {
            0 => accum - 1,
            1 => accum + 1,
            _ => accum,
        }
    });
    if result == 0 {
        return default_at_0;
    }
    (result < 0) as i32
}

fn calculate_power_consumption(input_path: &str) -> i32 {
    let str_data = read_input(input_path);

    let accum_diagnostic: Diagnostic<BIT_COUNT> = str_data
        .split('\n')
        .map(|str_data| {
            Diagnostic::<BIT_COUNT>::values_from_str(str_data)
                .ok()
                .unwrap()
        })
        .fold(Diagnostic::<BIT_COUNT>::default(), |acc, diag| {
            acc.add(diag)
        });

    let gamma = accum_diagnostic.gamma_rate();
    let epsilon = accum_diagnostic.epsilon_rate();

    gamma * epsilon
}

fn calculate_oxygen_rating(diagnostics: &Vec<Diagnostic<BIT_COUNT>>) -> i32 {
    let mut d = diagnostics.clone();
    let mut bit_flag_position = 0;
    while d.len() != 1 {
        let most_common_bit = most_common_value_at_pos(&d, bit_flag_position, 1);
        d.retain(|x| x.accum[bit_flag_position as usize] == most_common_bit );
        bit_flag_position += 1;
    }
    let mvp_diagnostics = d.first().unwrap().accum.clone();
    println!("oxygen: {:?}",mvp_diagnostics);
    Diagnostic::<BIT_COUNT>::vec_to_integer(mvp_diagnostics)
}

fn calculate_co2_rating(diagnostics: &Vec<Diagnostic<BIT_COUNT>>) -> i32 {
    let mut d = diagnostics.clone();
    let mut bit_flag_position = 0;
    while d.len() != 1 {
        let most_common_bit = least_common_value_at_pos(&d, bit_flag_position, 0);
        d.retain(|x| x.accum[bit_flag_position as usize] == most_common_bit );
        bit_flag_position += 1;
    }
    let mvp_diagnostics = d.first().unwrap().accum.clone();
    println!("co2: {:?}",mvp_diagnostics);
    Diagnostic::<BIT_COUNT>::vec_to_integer(mvp_diagnostics)
}

fn calculate_life_support(input_path: &str) -> i32 {
    let str_data = read_input(input_path);
    // Parse input
    let diagnostics: Vec<Diagnostic<BIT_COUNT>> = str_data
        .split('\n')
        .map(|str_data| {
            Diagnostic::<BIT_COUNT>::new(
                Diagnostic::<BIT_COUNT>::values_from_str(str_data)
                    .ok()
                    .unwrap(),
            )
        })
        .collect();
    let oxygen_rating = calculate_oxygen_rating(&diagnostics);
    let co2_rating = calculate_co2_rating(&diagnostics);
    oxygen_rating * co2_rating
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
