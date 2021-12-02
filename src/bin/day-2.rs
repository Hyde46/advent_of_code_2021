use std::error::Error;
use std::fmt;
use std::fs;
use std::ops::{Add, Mul, Neg};
use std::str::FromStr;

const INPUT: &str = "src/inputs/day-2.txt";

#[derive(Debug)]
struct Pos<T> {
    x: T,
    y: T,
    aim: T,
}

impl<'a, 'b, T> Add<&'b Pos<T>> for &'a Pos<T>
where
    &'a T: Add<&'b T, Output = T>,
    T: Copy,
{
    type Output = Pos<T>;
    fn add(self, other: &'b Pos<T>) -> Pos<T> {
        Pos {
            x: &self.x + &other.x,
            y: &self.y + &other.y,
            aim: self.aim,
        }
    }
}

impl<T> Pos<T> {
    fn new(x: T, y: T, aim: T) -> Pos<T> {
        Self {
            x: x,
            y: y,
            aim: aim,
        }
    }

    fn position_hash(self) -> T
    where
        T: Mul<Output = T>,
    {
        self.x * self.y
    }

    fn from_str(string: &str) -> Result<Pos<T>, AOCError>
    where
        T: FromStr,
        T: Default,
        T: Neg<Output = T>,
        <T as FromStr>::Err: std::fmt::Debug,
    {
        let mut value = string.split(" ");
        let lhs = value.next().unwrap();
        let rhs = value.next().unwrap();
        let rhs_value = rhs.parse::<T>().ok().unwrap();
        let pos = match lhs {
            "forward" => Pos {
                x: rhs_value,
                y: T::default(),
                aim: T::default(),
            },
            "up" => Pos {
                x: T::default(),
                y: -rhs_value,
                aim: T::default(),
            },
            "down" => Pos {
                x: T::default(),
                y: rhs_value,
                aim: T::default(),
            },
            _ => return Err(AOCError::new("Invalid instruction found")),
        };
        Ok(pos)
    }

    fn aim(self, other: &Pos<T>) -> Pos<T>
    where
        T: Add<Output = T>,
        T: Mul<Output = T>,
        T: Copy,
        T: PartialOrd,
        T: Default,
    {
        // Could generalize the aiming method to be part of add() if generic T would implement a Trait for identity value
        Pos {
            x: self.x + other.x,
            y: self.y + self.aim * other.x,
            aim: self.aim + other.y,
        }
    }
}

fn read_input(input: &str) -> Vec<Pos<i32>> {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let vectorized: Vec<Pos<i32>> = str_data
        .split("\n")
        .map(|string| Pos::from_str(string).ok().unwrap())
        .collect();
    vectorized
}

fn solve_dive(input_path: &str) -> i32 {
    let inputs: Vec<Pos<i32>> = read_input(input_path);
    inputs
        .iter()
        .fold(Pos::new(0, 0, 0), |acc, i| acc.add(i))
        .position_hash()
}

fn solve_aimed_dive(input_path: &str) -> i32 {
    let inputs: Vec<Pos<i32>> = read_input(input_path);
    inputs
        .iter()
        .fold(Pos::new(0, 0, 0), |acc, i| acc.aim(i))
        .position_hash()
}

fn main() {
    println!("Diving result: {}", solve_dive(INPUT));
    println!("Diving result: {}", solve_aimed_dive(INPUT));
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
