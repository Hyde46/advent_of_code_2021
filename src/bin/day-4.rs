use std::fs;
use std::iter::Iterator;

const INPUT: &str = "src/inputs/day-4.txt";

#[derive(Debug)]
struct BingoBoard {
    // Would be better to save which numbers are still
    // missing for the board to be a winning board. Would make this much much faster and efficient
    values: Vec<i32>,

    marked_values: Vec<i32>,
}

impl BingoBoard {
    fn new(values: Vec<i32>) -> BingoBoard {
        BingoBoard {
            values,
            marked_values: Vec::new(),
        }
    }

    fn from_iter(values_iter: &mut dyn Iterator<Item = &String>) -> BingoBoard {
        let mut board = BingoBoard::new(Vec::new());
        values_iter.for_each(|v_s| {
            let mut bingo_values: Vec<i32> = v_s
                .split(' ')
                .filter(|v| !v.is_empty())
                .map(|bingo_str_value| bingo_str_value.parse::<i32>().unwrap())
                .collect();
            board.values.append(&mut bingo_values);
        });
        board
    }

    fn get_row_values(&self, row: i32) -> Vec<i32> {
        let mut values = Vec::new();
        for i in 0..5 {
            let offset = i + (row * 5);
            values.push(self.values[offset as usize]);
        }
        values
    }

    fn get_column_values(&self, col: i32) -> Vec<i32> {
        let mut values: Vec<i32> = Vec::new();
        for i in 0..5 {
            let index = (col + (i * 5)) as usize;
            values.push(self.values[index]);
        }
        values
    }

    fn did_win_vertical(&mut self, values: &Vec<i32>) -> bool {
        let mut number_counter = 0;
        for i in 0..5 {
            let column = self.get_column_values(i);
            for mut col_value in column {
                if values.contains(&mut col_value) {
                    number_counter += 1;
                }
            }
            if number_counter == 5 {
                return true;
            }
            number_counter = 0;
        }
        false
    }

    fn did_win_horizontal(&mut self, values: &Vec<i32>) -> bool {
        let mut number_counter = 0;
        for i in 0..5 {
            let row = self.get_row_values(i);
            for mut row_value in row {
                if values.contains(&mut row_value) {
                    number_counter += 1;
                }
            }
            if number_counter == 5 {
                return true;
            }
            number_counter = 0;
        }
        false
    }

    fn did_win(&mut self, values: &Vec<i32>) -> bool {
        self.did_win_vertical(values) || self.did_win_horizontal(values)
    }

    fn winning_value(&self, values: &Vec<i32>) -> i32 {
        let left_numbers_sum: i32 = self
            .values
            .iter()
            .filter(|v| !self.marked_values.contains(v))
            .sum();
        let last_called_number = values.last().unwrap();
        left_numbers_sum * last_called_number
    }
}

fn read_input(input: &str) -> (Vec<i32>, Vec<BingoBoard>) {
    let str_data = fs::read_to_string(input).expect("Unable to open input");
    let mut split_string: Vec<String> = str_data
        .split("\n")
        .map(|s| s.to_owned())
        .filter(|x| !x.is_empty())
        .collect();
    // Prepare winning numbers
    let numbers: Vec<i32> = split_string[0]
        .clone()
        .split(',')
        .map(|s| s.parse::<i32>().unwrap())
        .collect();
    split_string.remove(0);
    // Parse Bingo boards
    let mut boards: Vec<BingoBoard> = Vec::new();
    while split_string.len() > 0 {
        // Get 5x5 bingo board values
        let mut board_iter = split_string.iter().take(5);
        // Create bingo board from values
        boards.push(BingoBoard::from_iter(&mut board_iter));
        // Remove used values from vector
        for _ in 0..5 {
            split_string.remove(0);
        }
    }
    (numbers, boards)
}

fn find_bingo_winner(input_path: &str) -> i32 {
    let (numbers, mut bingo_boards): (Vec<i32>, Vec<BingoBoard>) = read_input(input_path);
    let mut current_drawn_numbers: Vec<i32> = Vec::new();
    for val in numbers.iter() {
        current_drawn_numbers.push(*val);

        for board in &mut bingo_boards {
            board.marked_values.push(*val);
            if board.did_win(&current_drawn_numbers) {
                return board.winning_value(&current_drawn_numbers);
            }
        }
    }
    0
}

fn find_last_bingo_winner(input_path: &str) -> i32 {
    let (numbers, mut bingo_boards): (Vec<i32>, Vec<BingoBoard>) = read_input(input_path);
    let mut current_drawn_numbers: Vec<i32> = Vec::new();
    let board_len = bingo_boards.len();
    let mut boards_won: Vec<i32> = Vec::new();

    for val in numbers.iter() {
        current_drawn_numbers.push(*val);

        let mut index_counter = 0;
        for board in &mut bingo_boards {
            board.marked_values.push(*val);
            if boards_won.contains(&index_counter) {
                index_counter += 1;
                continue;
            }
            if board.did_win(&current_drawn_numbers) {
                if !boards_won.contains(&index_counter) {
                    boards_won.push(index_counter);
                }
                if boards_won.len() == board_len {
                    // Find last board and get value
                    let last_board_index = boards_won.last().unwrap().clone();
                    return bingo_boards[last_board_index as usize].winning_value(&current_drawn_numbers);
                }
            }
            index_counter += 1;
        }
    }
    0
}

fn main() {
    println!("Bingo winning board result: {}", find_bingo_winner(INPUT));
    println!("Last winning board result: {}", find_last_bingo_winner(INPUT));
}
