use itertools::zip;
use std::collections::HashMap;
use std::fs;

use regex::Regex;

const INPUT: &str = "src/inputs/day-5-example.txt";

#[derive(Debug, Clone)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Line {
    start: Point,
    end: Point,
}

impl Line {
    fn from_str(s: &str) -> Line {
        let re = Regex::new(r"^(\d{1,3}),(\d{1,3}) -> (\d{1,3}),(\d{1,3})").unwrap();
        let cap = re.captures(s).unwrap();
        Line {
            start: Point {
                x: cap
                    .get(1)
                    .map_or(-1, |m| m.as_str().parse::<i32>().unwrap()),
                y: cap
                    .get(2)
                    .map_or(-1, |m| m.as_str().parse::<i32>().unwrap()),
            },
            end: Point {
                x: cap
                    .get(3)
                    .map_or(-1, |m| m.as_str().parse::<i32>().unwrap()),
                y: cap
                    .get(4)
                    .map_or(-1, |m| m.as_str().parse::<i32>().unwrap()),
            },
        }
    }

    fn is_horizontal(&self) -> bool {
        self.start.y == self.end.y
    }

    fn is_vertical(&self) -> bool {
        self.start.x == self.end.x
    }

    fn is_diagonal(&self) -> bool {
        !self.is_horizontal() && !self.is_vertical()
    }

    fn get_coordinates_vec(&self) -> Vec<Point> {
        if self.is_vertical() {
            let (start, end) = super_complex_sort(self.start.y, self.end.y);
            return ((start)..(end + 1))
                .map(|y| Point { x: self.start.x, y })
                .collect();
        } else if self.is_horizontal() {
            let (start, end) = super_complex_sort(self.start.x, self.end.x);
            return ((start)..(end + 1))
                .map(|x| Point { x, y: self.start.y })
                .collect();
        } else {
            let range_x = produce_diagonal_range(self.start.x, self.end.x);
            let range_y = produce_diagonal_range(self.start.y, self.end.y);

            let mut vecs: Vec<Point> = Vec::new();
            for (x, y) in zip(range_x, range_y) {
                vecs.push(Point { x, y });
            }
            vecs
        }
    }
}

fn produce_diagonal_range(start: i32, end: i32) -> Box<dyn Iterator<Item = i32>> {
    if start < end {
        Box::new(start..(end + 1)) as Box<dyn Iterator<Item = i32>>
    } else {
        Box::new((end..(start + 1)).rev()) as Box<dyn Iterator<Item = i32>>
    }
}

fn super_complex_sort(first: i32, second: i32) -> (i32, i32) {
    if first < second {
        (first, second)
    } else {
        (second, first)
    }
}

fn read_input(input: &str) -> Vec<Line> {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let vectorized: Vec<Line> = str_data
        .split("\n")
        .map(|string| Line::from_str(string))
        .collect();
    vectorized
}

fn solve_simple_overlaps(input: &str) -> i32 {
    let lines = read_input(input);
    let non_diagonal_lines: Vec<Line> = lines
        .into_iter()
        .filter(|line| !line.is_diagonal())
        .collect();

    let mut points = Vec::<Point>::new();
    non_diagonal_lines
        .iter()
        .map(|line| line.get_coordinates_vec())
        .for_each(|vecs| points.append(&mut vecs.clone()));

    let mut m: HashMap<(i32, i32), i32> = HashMap::new();

    points.iter().for_each(|point| {
        let p = (point.x, point.y);
        m.insert(p, if m.contains_key(&p) { m[&p] + 1 } else { 1 });
    });
    m.into_iter()
        .fold(0, |acc, (_k, v)| if v >= 2 { acc + 1 } else { acc })
}

fn solve_complex_overlaps(input: &str) -> i32 {
    let lines = read_input(input);

    let mut points = Vec::<Point>::new();
    lines
        .iter()
        .map(|line| line.get_coordinates_vec())
        .for_each(|vecs| points.append(&mut vecs.clone()));

    let mut m: HashMap<(i32, i32), i32> = HashMap::new();

    points.iter().for_each(|point| {
        let p = (point.x, point.y);
        m.insert(p, if m.contains_key(&p) { m[&p] + 1 } else { 1 });
    });
    m.into_iter()
        .fold(0, |acc, (_k, v)| if v >= 2 { acc + 1 } else { acc })
}
fn main() {
    println!(
        "Hydrothermal simple overlaps: {}",
        solve_simple_overlaps(INPUT)
    );
    println!(
        "Hydrothermal complex overlaps: {}",
        solve_complex_overlaps(INPUT)
    );
}
