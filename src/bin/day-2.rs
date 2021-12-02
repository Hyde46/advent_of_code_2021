use std::ops::{Add, Mul, Neg};
use std::fs;
use std::str::FromStr;
use std::error::Error;
use std::fmt;

const INPUT: &str = "src/inputs/day-2.txt";


#[derive(Debug)]
struct Pos<T> {
    x: T,
    y: T
}

impl<T> Pos<T> {
    fn new(x: T, y: T) -> Pos<T> { Self {x: x, y: y}}

    fn position_hash(self) -> T
        where T: Mul<Output=T>
    {
        self.x * self.y
    }

    fn from_str(string: &str) -> Result<Pos<T>, AOCError> 
        where T: FromStr, T: Default, T: Neg<Output = T>, <T as FromStr>::Err : std::fmt::Debug
    {
        let mut value = string.split(" ");
        let lhs = value.next().unwrap();
        let rhs = value.next().unwrap();
        let rhs_value = rhs.parse::<T>().ok().unwrap();
        let pos = match lhs {
            "forward" => {
                Pos{ x: rhs_value, y: T::default()}
            },
            "up" => {
                Pos{ x: T::default(), y: -rhs_value}
            },
            "down" => {
                Pos{ x: T::default(), y: rhs_value}
            },
            _ => {
                return Err(AOCError::new("Invalid instruction found"))
            }
        };
        Ok(pos)
    }
}

impl<'a, 'b, T> Add<&'b Pos<T>> for &'a Pos<T>
    where &'a T: Add<&'b T, Output=T>
{
    type Output = Pos<T>;
    fn add(self, other: &'b Pos<T>) -> Pos<T> {
        Pos {
            x: &self.x + &other.x,
            y: &self.y + &other.y
        }
    }
}

fn read_input(input: &str) -> Vec<Pos<i32>> {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let vectorized: Vec<Pos<i32>> = str_data.split("\n").map( | string | Pos::from_str(string).ok().unwrap() ).collect();
    vectorized
}

fn solve_dive(input_path: &str) -> i32 {
    let inputs: Vec<Pos<i32>> = read_input(input_path);
    inputs.iter().fold(Pos::new(0,0), |acc, i| acc.add(i)).position_hash()
}


fn main() {
    println!("Diving result: {}", solve_dive(INPUT))
}

#[derive(Debug)]
struct AOCError {
    details: String
}

impl AOCError {
    fn new(msg: &str) -> AOCError {
        AOCError{details: msg.to_string()}
    }
}

impl fmt::Display for AOCError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f,"{}",self.details)
    }
}

impl Error for AOCError {
    fn description(&self) -> &str {
        &self.details
    }
}